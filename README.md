# puc-2023

## Task
### EN
This project contains a file handler that has 4 functions open_file, read_file, write_file and close_file

Constraints are:
- read and write can only be called after open and before close.
- There can be multiple reads but only one write on a file.
- File handlers should not leak, that means they need to be closed when they get out of scope

All this should be possible with the rust ownership model

### DE
Sie haben eine API mit 4 Funktionen. open_file, read_file, write_file und close_file.

- read und write dürfen nur nach open und vor close aufgerufen werden.
- es dürfen mehrere reads parallel ausgeführt werden aber immer nur ein write.
- Es sollen keine File handles geleakt werden, das heißt wenn ein handle aus dem scope geht soll er geschlossen werden.

All diese Anforderungen lassen sich mit Rusts Ownership system abdecken. 
