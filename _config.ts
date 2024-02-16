import lume from "lume/mod.ts";

import codeHighlight from "lume/plugins/code_highlight.ts";
import esbuild from "lume/plugins/esbuild.ts";
import feed from "lume/plugins/feed.ts";
import jsx from "lume/plugins/jsx.ts";
import minifyHTML from "lume/plugins/minify_html.ts";

const site = lume({
  src: "./site",
});

site.use(codeHighlight());
site.use(esbuild());
site.use(jsx());
site.use(minifyHTML());

site.use(feed({
  output: ["/blog.rss"],

  query: "type=article",
  sort: "date=desc",

  info: {
    title: "RGBCube's Blog",
    description:
      "The blog where RGBCube dumps his schizophrenic ramblings about software and all the likes.",
    lang: "en",
    generator: false,
  },
  items: {
    title: "=title",
    description: "=description",
    published: "=date",
    content: "$ content",
    lang: "en",
  },
}));

site.copyRemainingFiles();

export default site;
