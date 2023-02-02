import React from "react";
import "./App.css";
import * as wasm from "poc-test-wasm";
import styled from 'styled-components';

wasm.default();

const canvaCtx = wasm.start("canvas");

function App() {

  const [canvaId, setCanvaId] = React.useState<string>("canvas");
  const [text, setText] = React.useState<string>("");
  const [x, setX] = React.useState<number>(50);
  const [y, setY] = React.useState<number>(50);


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
    wasm.start_drawing(canvaCtx);
    wasm.draw_line(canvaCtx, x, y);
    wasm.stop_drawing(canvaCtx);
  }

  const drawtext = () => {
    wasm.start_drawing(canvaCtx);
    wasm.draw_text(canvaCtx, text, x, y, 500);
    wasm.fill(canvaCtx);
    wasm.stop_drawing(canvaCtx);
  }

  const drawrect = () => {
    wasm.start_drawing(canvaCtx);
    wasm.draw_rect(canvaCtx, x, y, 20, 30);
    wasm.stroke(canvaCtx);
  }

  const setCyan = () => {
    wasm.set_color(canvaCtx, "cyan");
  }

  const setRed = () => {
    wasm.set_color(canvaCtx, "red");
  }

  const setGreen = () => {
    wasm.set_color(canvaCtx, "green");
  }

  const clearCanva = () => {
    wasm.clear_canva(canvaCtx, 300, 300);
  }

  const drawCircle = () => {
    wasm.start_drawing(canvaCtx);
    wasm.draw_circle(canvaCtx, x, y, 50);
    wasm.stop_drawing(canvaCtx);

  }


  const drawArc = () => {
    wasm.start_drawing(canvaCtx);
    wasm.draw_arc(canvaCtx, x, y, 50, 1, 2);
    wasm.fill(canvaCtx);
    wasm.stop_drawing(canvaCtx);
  }

  const drawBoxCircle = () => {
    wasm.start_drawing(canvaCtx);
    wasm.draw_circle(canvaCtx, x, y, 50);
    wasm.stop_drawing(canvaCtx);
    wasm.sleep_milliseconds(BigInt(500));
    wasm.start_drawing(canvaCtx);
    wasm.draw_rect(canvaCtx, x, y, 50, 60);
    wasm.stop_drawing(canvaCtx);
  }


  return (
    <Div className="App">
      <Wrapper>
        <Div>
          <input onChange={handleChange} placeholder="text"/>
          <input onChange={handleXPos} placeholder="x pos"/>
          <input onChange={handleYPos} placeholder="y pos"/>
          <button onClick={drawtext}>text</button>
          <button onClick={drawrect}>rect</button>
          <button onClick={setCyan}>cyan</button>
          <button onClick={setRed}>red</button>
          <button onClick={setGreen}>green</button>
          <button onClick={drawCircle}>circle</button>
          <button onClick={drawArc}>arc</button>
          <button onClick={clearCanva}>clear</button>
          <button onClick={drawBoxCircle}>boxCircle</button>
        </Div>
      </Wrapper>
      <Div>
        <Canvas id={canvaId} width="300" height="300">
        </Canvas>
      </Div>
    </Div>
  );
}

const Div = styled.div`
`

const Canvas = styled.canvas`
`

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: space-between;
`

export default App;
