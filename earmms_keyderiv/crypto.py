from nacl import pwhash, exceptions, secret, utils


ops = int(pwhash.SCRYPT_OPSLIMIT_INTERACTIVE)
mem = int(pwhash.SCRYPT_MEMLIMIT_INTERACTIVE)

INDEX_KEY = None
ENCRYPT_KEY = None

def set_index_key(key):
    global INDEX_KEY
    INDEX_KEY = key

def set_encrypt_key(key):
    global ENCRYPT_KEY
    ENCRYPT_KEY = key

def get_blind_index(data):
    if INDEX_KEY is None:
        raise RuntimeError("INDEX_KEY was not set when trying to get_blind_index")

    return pwhash.kdf_scryptsalsa208sha256(
        32,
        data,
        INDEX_KEY,
        opslimit=ops,
        memlimit=mem
    )

def get_encrypt_key(objid, field):
    if ENCRYPT_KEY is None:
        raise RuntimeError("ENCRYPT_KEY was not set when trying to get_encrypt_key")

    key = pwhash.kdf_scryptsalsa208sha256(
        secret.SecretBox.KEY_SIZE,
        b"%s:%s" % (objid, field),
        ENCRYPT_KEY,
        opslimit=ops,
        memlimit=mem
    )

    return key
