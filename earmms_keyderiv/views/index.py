import base64

from flask import jsonify, request
from flask.views import MethodView

from earmms_keyderiv.crypto import get_blind_index


class IndexView(MethodView):
    def post(self):
        ########################################################################
        # check for required fields

        data = request.get_json()

        if "data" not in data:
            return jsonify({"error": "data not specified"}), 400

        ########################################################################
        # generate blind index for given data

        index = get_blind_index(base64.b64decode(data["data"].encode("utf-8")))

        ########################################################################
        # return json with output

        output = {
            "index": base64.b64encode(index).decode('utf-8')
        }

        return jsonify(output)
