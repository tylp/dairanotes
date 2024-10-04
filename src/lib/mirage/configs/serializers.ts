import { Serializer } from "miragejs";

const UnwrappedSerializer = Serializer.extend({
  serialize(object) {
    if ("models" in object) return object.models.map((model) => model.attrs);

    return object;
  },
});

export const serializers = {
  note: UnwrappedSerializer,
};
