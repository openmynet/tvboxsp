import { Modal } from "@arco-design/web-vue";
function confirm(content: string) {
  return new Promise((ok, fail) => {
    Modal.confirm({
      content,
      onOk(e) {
        ok(e);
      },
      onCancel(e) {
        fail(e);
      },
    });
  });
}

export { confirm };
