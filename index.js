
const url = new URL(document.location.href);
const params = url.searchParams;
const id = params.get('id') ?? 1;

import.meta.glob('./crates/sketch_1/assets/**/*', {
  as: 'raw',
  eager: true,
});

import(`./crates/sketch_${id}/pkg/sketch_${id}.js`).then(
  async ({ default: sketch }) => {
    await sketch().then((s) => {
      s.main_web();
    });
  },
);
