# Copyright 2024 StarfleetAI
# SPDX-License-Identifier: Apache-2.0

import json
from dataclasses import dataclass, field
from typing import Annotated, Callable, Dict, Any, get_origin, get_args

@dataclass
class Ability:
    _functions: Dict[str, Dict[str, Any]] = field(default_factory=dict, init=False)

    def register(self) -> Callable:
        def decorator(func: Callable) -> Callable:
            annotations = func.__annotations__
            annotated_info = {}

            for param, anno in annotations.items():
                origin = get_origin(anno)
                args = get_args(anno)

                if origin is Annotated:
                    primary_type = args[0]
                    metadata = args[1] if len(args) > 1 else None
                    annotated_info[param] = {"type": primary_type, "metadata": metadata}
                else:
                    annotated_info[param] = {"type": anno, "metadata": None}

            self._functions[func.__name__] = {
                'annotations': annotated_info,
                'function': func
            }
            return func

        return decorator

    def functions_definitions(self, only=None):
        funcs = self._functions.keys()
        if only:
            funcs = [func for func in funcs if func in only]

        return [self._function_definition(func_name) for func_name in funcs]

    def _function_definition(self, func_name):
        func_info = self._functions.get(func_name)
        if not func_info:
            raise ValueError(f"Function {func_name} not registered")

        properties = {}
        for param, anno in func_info['annotations'].items():
            if param == 'return':
                continue

            properties[param] = {
                'type': self._to_json_schema_type(anno['type'])
            }
            if anno['metadata']:
                properties[param]['description'] = anno['metadata']

        return {
            'type': 'function',
            'function': {
                'name': func_name,
                'parameters': {
                    'type': 'object',
                    'properties': properties
                }
            }
        }

    def _to_json_schema_type(self, type):
        if type is str:
            return 'string'
        elif type is int:
            return 'integer'
        elif type is float:
            return 'number'
        elif type is bool:
            return 'boolean'
        elif type is list:
            return 'array'
        elif type is dict:
            return 'object'
        else:
            return 'string'

ability = Ability()

@ability.register()
{{ code }}

print(json.dumps(ability.functions_definitions()[0]))

