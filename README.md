A walk-through codebase of [Rust Full Stack Workshop](https://bcnrust.github.io/devbcn-workshop/index.html)

### Debugging using Visual Studio Code

Make sure that you have these two extensions installed on your editor:

- Rust Analyzer
- CodeLLDB

Create a new file in the root of your project called `.vscode/launch.json`, and write the configuration as follows:

```json
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "attach",
      "name": "Attach to Shuttle",
      "program": "${workspaceFolder}/target/debug/api-shuttle"
    }
  ]
}
```

The most important point to take into account here is that the program attribute must point to the binary that you want to debug.

Now it's ready.

0. On your VSCode, put a breakpoint anywhere you want to inspect.
1. Run your dev server (in this example, `cargo shuttle run`).
2. Then press `F5` (on macos, `fn + F5`) to start debugging.
