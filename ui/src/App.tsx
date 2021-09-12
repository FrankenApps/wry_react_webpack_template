import React from 'react';

// This function can be used to communicate with the Rust backend.
declare function wry(header: string, message: any): Promise<any>

export default function App()
{
    const [greetTitle, setGreetTitle] = React.useState("Me");
    const [textContent, setTextContent] = React.useState("");

    return (
        <div>
            <h1>Hello, World!</h1>
            <button onClick={() => wry("minimize", {})}>Minimize</button>
            <br></br>
            <div style={{ display: "inline" }}>
                <input type="text" 
                       onChange={(event) => setGreetTitle(event.target.value)}
                       value={greetTitle}></input>
                <button onClick={() => wry("greet", { title: greetTitle })}
                        title="Greets you from the native side.">Greet</button>
            </div>
            <br></br>
            <button onClick={() => {
                wry("open", {}).then((result) => {
                    setTextContent(result.content);
                });
            }}>Open file</button>
             <br></br>
            <p>{textContent}</p>
        </div>
    );
}