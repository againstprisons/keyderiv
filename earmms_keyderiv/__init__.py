import os

from flask import Flask

from earmms_keyderiv.crypto import set_index_key, set_encrypt_key

def app_factory():
    app = Flask(__name__)

    ############################################################################
    # load app configs

    if os.environ.get("EARMMS_KEYDERIV_CONFIG", None) not in ["", None]:
        app.config.from_envvar("EARMMS_KEYDERIV_CONFIG")

    ############################################################################
    # load our encryption keys

    set_index_key(app.config.get("INDEX_KEY"))
    set_encrypt_key(app.config.get("ENCRYPT_KEY"))

    ############################################################################
    # register views

    from earmms_keyderiv.views.index import IndexView
    app.add_url_rule("/index", view_func=IndexView.as_view("index"))

    from earmms_keyderiv.views.encrypt_key import EncryptKeyView
    app.add_url_rule("/encryptkey", view_func=EncryptKeyView.as_view("encryptkey"))

    ############################################################################
    # return the application object

    return app
