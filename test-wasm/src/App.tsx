import React from "react";
import "./App.css";
import * as wasm from "poc-test-wasm";

wasm.default();

const canva_ctx = wasm.start();

const draw = () =>  {
  wasm.draw_face(canva_ctx);
}

let x = 50;
let y = 50;

function App() {

  const [text, setText] = React.useState("");
  const [x, setX] = React.useState(50);
  const [y, setY] = React.useState(50);


  const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
    setText(event.currentTarget.value);
  };
  const handleXPos = (event: React.FormEvent<HTMLInputElement>) => {
    setX(parseInt(event.currentTarget.value));
  };
  const handleYPos = (event: React.FormEvent<HTMLInputElement>) => {
    setY(parseInt(event.currentTarget.value));
  };

  const drawline = () =>  {
    wasm.draw_line(canva_ctx, x, y);
  }

  const drawtext = () =>  {
    wasm.draw_text(canva_ctx, text, x, y, 300);
  }

  const drawrect = () =>  {
    wasm.draw_rect(canva_ctx, x, y, 20, 30, false);
  }

  const setCyan = () =>  {
    wasm.set_color(canva_ctx, "cyan");
  }

  const setRed = () =>  {
    wasm.set_color(canva_ctx, "red");
  }

  const setGreen = () =>  {
    wasm.set_color(canva_ctx, "green");
  }

  const drawCircle = () =>  {
    wasm.draw_circle(canva_ctx, x, y, 50);
  }

  const clearCanva = () =>  {
    setX(50);
    setY(50);
    wasm.clear_canva(canva_ctx, 500, 500);
  }

  const drawArc = () =>  {
    wasm.draw_arc(canva_ctx, x, y, 50, 1, 2);
  }


  return (
    <div className="App">
      <input onChange={handleChange} placeholder="text"/>
      <input onChange={handleXPos} placeholder="x pos"/>
      <input onChange={handleYPos} placeholder="y pos"/>
      <button onClick={draw}>draw face</button>
      <button onClick={drawline}>line</button>
      <button onClick={drawtext}>text</button>
      <button onClick={drawrect}>rect</button>
      <button onClick={setCyan}>cyan</button>
      <button onClick={setRed}>red</button>
      <button onClick={setGreen}>green</button>
      <button onClick={drawCircle}>circle</button>
      <button onClick={clearCanva}>clear</button>
      <button onClick={drawArc}>arc</button>
      <div>
        <canvas id="canvas" width="300" height="300">
        </canvas>
      </div>
    </div>
  );
}


export default App;
