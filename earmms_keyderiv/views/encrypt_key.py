import base64

from flask import jsonify, request
from flask.views import MethodView

from earmms_keyderiv.crypto import get_encrypt_key


class EncryptKeyView(MethodView):
    def post(self):
        ########################################################################
        # check for required fields

        data = request.get_json()

        required = [
            "rowid",
            "field"
        ]

        for field in required:
            if field not in data:
                return jsonify({"error": "%s not specified" % field}), 400

        ########################################################################
        # generate encryption key

        key = get_encrypt_key(
            data["rowid"].encode('utf-8'),
            data["field"].encode('utf-8')
        )

        ########################################################################
        # return json with output

        output = {
            "key": base64.b64encode(key).decode('utf-8')
        }

        return jsonify(output)
