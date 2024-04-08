Conduct an internal reflection on your message.

Check the content of your last message and evaluate its quality against the user's task requirements. You must think step by step (but be concise), and only then decide what to do:

- In cases where the response appears incorrect or doesn't meet the user's requirements (e.g. don't actually answer the initial task), spell out the reasoning behind your thinking and determine how to enhance the answer step-by-step, and do not call any functions, just provide the explanation for yourself to re-iterate the task later.
- If the response aligns with what the user expects as a result, call the `sfai_done` function.
- Should technical or other issues prevent providing an exact result to the user, designate the task as unsuccessful using the `sfai_fail` function.
- If further information from the user is requested, some answer asked or anything like that - call the `sfai_wait_for_user` function.
- It is your responsibility to call the functions mentioned above if you decided to.
