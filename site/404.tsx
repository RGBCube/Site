import Cube from "./_includes/cube.tsx";

export const layout = "base.vto";
export const title = "404";

export default (_data: Lume.Data, helpers: Lume.Helpers) => {
  const face = (
    <>
      <div className="frame">
        <a href={helpers.url("/", true)}>
          404
        </a>
      </div>
      <div className="square black"></div>
      <div className="square magenta"></div>
      <div className="square magenta"></div>
      <div className="square black"></div>
    </>
  );

  return (
    <Cube
      front={face}
      back={face}
      left={face}
      right={face}
      top={face}
      bottom={face}
    />
  );
};
