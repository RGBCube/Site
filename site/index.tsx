import Cube from "./_includes/cube.tsx";

export const title = "RGBCube";

export default (
  <Cube
    front={
      <a href="/about">
        <div className="frame">about</div>
      </a>
    }
    top={
      <a href="https://github.com/RGBCube">
        <div className="frame">github</div>
      </a>
    }
    right={
      <a href="/contact">
        <div className="frame">contact</div>
      </a>
    }
    left={
      <a href="/blog">
        <div className="frame">blog</div>
      </a>
    }
  />
);
