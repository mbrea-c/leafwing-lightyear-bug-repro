# How to reproduce bug

1. Start the server and client (the `run_2_players.py` script does that for you)
1. There's a `TestAction::Test` leafwing action bound to the left mouse button, and a system running on `FixedUpdate` that will print the current tick whenever that action has been `.just_pressed()`.
2. The bug does not happen every time you click, only if it coincides with a rollback. There's a system triggering rollbacks every time components are synced, so just try a few times and you should see something like this:
```
CLIENT: Action pressed in tick Tick(2068)
CLIENT: Action pressed in tick Tick(2068)
SERVER: Action pressed in tick Tick(2068)
CLIENT: Action pressed in tick Tick(2069)
```
