import lume from "lume/mod.ts";

import codeHighlight from "lume/plugins/code_highlight.ts";
import esbuild from "lume/plugins/esbuild.ts";
import feed from "lume/plugins/feed.ts";
import jsx from "lume/plugins/jsx.ts";
import minifyHTML from "lume/plugins/minify_html.ts";
import sitemap from "lume/plugins/sitemap.ts";

const site = lume({
  src: "./site",
});

site.use(codeHighlight());
site.use(esbuild());
site.use(jsx());
site.use(minifyHTML());

site.process([".html"], (pages) => {
  pages.forEach((page) => {
    const document = page.document!;

    document.querySelectorAll("table").forEach((table) => {
      const div = document.createElement("div");

      div.classList.add("room", "rotated");
      table.classList.add("rotated");

      table.parentNode!.insertBefore(div, table);
      div.appendChild(table);
    });
  });
});

site.use(feed({
  output: ["/blog.rss"],

  query: "type=article",
  sort: "date=asc",
  limit: Infinity,

  info: {
    title: "RGBCube's Blog",
    description:
      "The blog where RGBCube dumps his schizophrenic ramblings about software and all the likes.",
    generator: false,
  },
  items: {
    title: "=title",
    description: "=description",
    published: "=date",
    content: "$.content",
  },
}));

site.use(sitemap({
  // @ts-ignore: We don't want lastmods.
  lastmod: null,
}));

site.copyRemainingFiles();

export default site;
