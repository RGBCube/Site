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
  <>
    <style
      dangerouslySetInnerHTML={{
        __html: `
      html {
        font-size: min(9vw, 9vh, 4.5em);
        overscroll-behavior: none;
      }

      a {
        color: black;
        font-weight: bold;
      }

      .frame {
        background-color: white;

        width: min-content;

        padding: 0 0.3em;
        border-radius: 1em;

        user-select: none;
      }

      .frame:hover {
        background-color: yellow;
      }

      .scene {
        height: 100dvh;
        width: 100dvw;
        perspective: 15em;

        display: flex;

        align-items: center;
        justify-content: center;
      }

      .cube {
        height: 5em;
        width: 5em;

        position: relative;

        transform: translateZ(-calc(2.5em - 1px));
        transform-style: preserve-3d;
      }

      .face {
        width: 5em;
        height: 5em;

        display: flex;

        align-items: center;
        justify-content: center;

        position: absolute;
      }

      .front {
        transform: rotateY(0deg) translateZ(calc(2.5em - 1px));
      }

      .top {
        /* Guess what? Yeah, you guessed right. Safari can't render shit. */
        transform: rotateX(89.99999999999999deg) translateZ(calc(2.5em - 1px));
      }

      .back {
        transform: rotateY(180deg) translateZ(calc(2.5em - 1px));
      }

      .bottom {
        transform: rotateX(-89.99999999999999deg) translateZ(calc(2.5em - 1px));
      }

      .right {
        transform: rotateY(89.99999999999999deg) translateZ(calc(2.5em - 1px));
      }

      .left {
        transform: rotateY(-89.99999999999999deg) translateZ(calc(2.5em - 1px));
      }
    `,
      }}
    >
    </style>

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

    <script src="/assets/cube.js"></script>
  </>
);

export default Cube;
