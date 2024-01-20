import lume from "lume/mod.ts";

import codeHighlight from "lume/plugins/code_highlight.ts";
import feed from "lume/plugins/feed.ts";
import inline from "lume/plugins/inline.ts";
import minifyHTML from "lume/plugins/minify_html.ts";
import resolveUrls from "lume/plugins/resolve_urls.ts";

const site = lume({
  src: "./site",
  location: new URL("https://rgbcu.be/"),
});

site.use(codeHighlight());
site.use(inline());
site.use(resolveUrls());
site.use(minifyHTML());

site.use(feed({
  output: ["/blog/feed.rss"],

  query: "type=article",
  sort: "date=desc",

  info: {
    title: "RGBCube's Blog",
    description:
      "The webpage where RGBCube dumps his schizophrenic ramblings about software and all the likes.",
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
