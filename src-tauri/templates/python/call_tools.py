# Copyright 2024 StarfleetAI
# SPDX-License-Identifier: Apache-2.0

import json
from typing import Annotated, Callable, Dict, Any, get_origin, get_args

{{ code }}

tool_call = {{ tool_call }}
name = tool_call['function']['name']
try:
    arguments = json.loads(tool_call['function']['arguments'])
    print(globals()[name](**arguments))
except Exception as e:
    print(str(e))
