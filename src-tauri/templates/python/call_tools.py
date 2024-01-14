# Copyright 2024 StarfleetAI
# SPDX-License-Identifier: Apache-2.0

import json
from typing import Annotated, Callable, Dict, Any, get_origin, get_args

PYTHON_PATH = '{{ python_path }}'

{{ code }}

tool_calls = {{ tool_calls }}
results = {}
for tool_call in tool_calls:
    name = tool_call['function']['name']
    arguments = json.loads(tool_call['function']['arguments'])

    try:
        results[tool_call['id']] = globals()[name](**arguments)
    except Exception as e:
        results[tool_call['id']] = str(e)
        break

print(json.dumps(results))
