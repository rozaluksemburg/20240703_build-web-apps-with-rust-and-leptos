# Setup a Leptos Starter Project

1. Open a terminal window, and change in to the `projects` folder.

```bash
cd projects
```

2. Within the `projects` folder, run the following command.

```bash
cargo leptos new --git leptos-rs/start
```

3. The setup tool will ask for a project name, type `tools-app`.

4. Add the project path `projects/tools-app` to the `members` list in the root folder `Cargo.toml` file.

5. Change into the `tools-app` folder. Type the following command to run the project in watch mode.

короче - тут нужно внести наш форк-проект, который в workspace github, в корневой Cargo.toml проекта таким образом
members = ["projects/tools-app-final", "projects/tools-app"]
а изначально было 
members = ["projects/tools-app-final"]

```bash
команда приведет к режиму просмотра и в ходе редактирования файлов cargo letpos watch будет производить 
так называемую горячее пересобирание всего проекта в режиме реального времени 
cargo leptos watch 
но лучше это не включать в ходе разработки, потому что в реальности в каких-то моментах это может дать осечку 
поэтому лучше редактировать файлы без включенного cargo leptos watch
лучше использовать 
cargo leptos serve
но лучше выключать и его при редактировании файлов, а потом снова включать, то есть просто перезагружать 
```

6. Once compilation is complete, open a web browser, and navigate to `http://localhost:3000`.
