import sys

import numpy as np
import tensorflow as tf
from tensorflow import keras

tf.compat.v1.logging.set_verbosity(tf.compat.v1.logging.ERROR)

class_names = ["Bad", "Good", "Loading"]

model = keras.models.load_model('image_model')

sc_dir = sys.argv[1]
inst_width = sys.argv[2]
inst_height = sys.argv[3]


def predict_img(file, width, height):
    """Predicts image into 1 of x classes

    Args:
        file (string): path to image

    Returns:
        string: the class the image falls into
    """
    img = tf.keras.utils.load_img(file, target_size=(height, width))
    img_array = tf.keras.utils.img_to_array(img)
    img_array = tf.expand_dims(img_array, 0)

    predictions = model.predict(img_array)
    score = tf.nn.softmax(predictions[0])
    return class_names[np.argmax(score)]

print(sc_dir, inst_width, inst_height)
