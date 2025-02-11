# Tasky

A simple terminal-based to-do list manager written in Rust. Tasky allows you to add, view, complete, and delete tasks while saving them between sessions.

Jira(not accesible by anyone else. for now.): https://theunconcernedape.atlassian.net/jira/software/projects/RS/boards/2

## Features
- Add tasks
- View all tasks
- Mark tasks as complete
- Delete tasks
- Automatically saves tasks to a file on exit
- Loads tasks from a file on startup

## Requirements
- Rust (latest stable version)

## Usage
Run the program and interact with the menu:
```
1. Add Task
2. View Tasks
3. Complete Task
4. Delete Task
5. Exit
```
Follow the prompts to manage your tasks.

## File Storage
Tasks are saved in a `tasks.json` file in the same directory as the program.

## Future Improvements
- Better error handling
- Colored output for better readability
- Sorting tasks by completion status
- arguements to skip the interface menu
- custom `tasks.json` file path / name

## License

[GNU General Public License v2.0](https://choosealicense.com/licenses/gpl-2.0/)
