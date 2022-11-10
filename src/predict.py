import sys
import os
import logging
import mouse

import numpy as np
import tensorflow as tf
from tensorflow import keras
import silence_tensorflow.auto

tf.compat.v1.logging.set_verbosity(tf.compat.v1.logging.ERROR)
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3'
tf.get_logger().setLevel(logging.ERROR)
tf.autograph.set_verbosity(1)

class_names = ["Bad", "Good", "Loading"]

model = keras.models.load_model('image_model')

sc_dir = sys.argv[1]
inst_width = int(sys.argv[2])
inst_height = int(sys.argv[3])
inst_x = int(sys.argv[4])
inst_y = int(sys.argv[5])
x_offset = inst_x + (inst_width / 2)
y_offset = inst_y + (inst_height / 2)


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
    res = class_names[np.argmax(score)]
    if res == "Bad":
        mouse.move(x_offset, y_offset)
        mouse.click()

predict_img(sc_dir, inst_width, inst_height)