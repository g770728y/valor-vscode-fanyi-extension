import axios from "axios";
import * as vscode from "vscode";
export function activate(context: vscode.ExtensionContext) {
  console.log('Congratulations, your extension "fanyi" is now active!');

  let disposable = vscode.commands.registerCommand("fanyi.start", () => {
    vscode.window
      .showInputBox({ placeHolder: "输入要翻译的中文词语" })
      .then((src) => {
        axios
          .get("http://1.14.96.38:8999/translate?q=" + encodeURI(src || ""))
          .then((r) =>
            vscode.window.showInputBox({
              value: r.data?.[0]?.dst || "查询结果为空",
              prompt: "查询结果, 可直接复制",
            })
          )
          .catch((e) => vscode.window.showErrorMessage(e));
      });
  });

  context.subscriptions.push(disposable);
}

export function deactivate() {}
