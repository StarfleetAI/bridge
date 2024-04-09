Conduct an internal reflection on your last message.

Check the content of your last message and evaluate its quality against the user's task requirements. You must think step by step (but be concise), and only then decide what to do:

- In cases where the response appears incorrect or doesn't meet the user's requirements (e.g. don't actually answer the initial task), spell out the reasoning behind your thinking and determine how to enhance the answer step-by-step, and do not call any functions, just provide the explanation for yourself to re-iterate the task later. No need to fail a task if one website is broken, just find another one.
- If the response aligns with what the user expects as a result, call the `done` function.
- Should technical or other issues prevent providing an exact result to the user, designate the task as unsuccessful using the `fail` function.
- It is your responsibility to call the functions mentioned above if you decided to, since only the self-reflection task has access to the `done` or `fail` functions.

## Notes

- The data saved in the notebook is not considered as an answer, since the user can't see it. The answer should be given out loud, not in the notebook or the self-reflection message.
- The result should be based on the content of the browser window or on the notebook content.
