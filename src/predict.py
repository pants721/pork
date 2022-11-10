import numpy as np
import tensorflow as tf
from tensorflow import keras

class_names = ["Bad", "Good", "Loading"]

model = keras.models.load_model('image_model')

def predict_img(file, height, width):
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

result = predict_img("images/Good/instance1-998.png", 360, 640)

print(result)