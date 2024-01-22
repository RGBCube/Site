import React, { ReactNode as Node } from "npm:react";

const empty: Node = <></>;

interface CubeProps {
  front?: Node;
  back?: Node;
  left?: Node;
  right?: Node;
  top?: Node;
  bottom?: Node;
}

const Cube = (props: CubeProps) => (
  <div className="scene">
    <div className="cube">
      <div className="face front">{props.front || empty}</div>
      <div className="face back">{props.back || empty}</div>
      <div className="face left">{props.left || empty}</div>
      <div className="face right">{props.right || empty}</div>
      <div className="face top">{props.top || empty}</div>
      <div className="face bottom">{props.bottom || empty}</div>
    </div>
  </div>
);

export default Cube;
