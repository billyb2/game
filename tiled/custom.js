var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        let array = [];;

        array.push((map.width * 15 & 0xff000000) >> 24);
        array.push((map.width * 15 & 0x00ff0000) >> 16);
        array.push((map.width * 15 & 0x0000ff00) >> 8);
        array.push((map.width * 15 & 0x000000ff));

        array.push((map.height * 15 & 0xff000000) >> 24);
        array.push((map.height * 15 & 0x00ff0000) >> 16);
        array.push((map.height * 15 & 0x0000ff00) >> 8);
        array.push((map.height * 15 & 0x000000ff));

        for (i = map.layerCount  - 1; i >= 0; i--) {
            var layer = map.layerAt(i);

            if (layer.isTileLayer) {
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.tileAt(x, y) != null) {
                            // x coor
                            array.push((x * 15 & 0xff000000) >> 24);
                            array.push((x * 15 & 0x00ff0000) >> 16);
                            array.push((x * 15 & 0x0000ff00) >> 8);
                            array.push((x * 15 & 0x000000ff));

                            //y coor
                            array.push((y * 15 & 0xff000000) >> 24);
                            array.push((y * 15 & 0x00ff0000) >> 16);
                            array.push((y * 15 & 0x0000ff00) >> 8);
                            array.push(y * 15 & 0x000000ff);

                            //width
                            array.push((15 & 0xff000000) >> 24);
                            array.push((15 & 0x00ff0000) >> 16);
                            array.push((15 & 0x0000ff00) >> 8);
                            array.push(15 & 0x000000ff);

                            //height
                            array.push((15 & 0xff000000) >> 24);
                            array.push((15 & 0x00ff0000) >> 16);
                            array.push((15 & 0x0000ff00) >> 8);
                           array.push(15 & 0x000000ff);

                            if (layer.tileAt(x, y).property("player_spawn") == true) {
                                array.push(255);

                            } else {
                                array.push(0);

                            }

                            if (layer.tileAt(x, y).property("player_collidable") == true) {
                                array.push(255);

                            } else {
                                array.push(0);

                            }

                            array.push(layer.tileAt(x, y).property("red"));
                            array.push(layer.tileAt(x, y).property("green"));
                            array.push(layer.tileAt(x, y).property("blue"));
                            array.push(layer.tileAt(x, y).property("alpha"));
                            array.push(0);


                            for (var z = 0; z < array.length; z += 23) {
                                var is_null = true;

                                for (var j = 0; j <= 22; j ++) {
                                    if (array[j + z] != 0) {
                                        is_null = false;
                                        break;

                                    }
                                }

                                if (is_null == true) {
                                    index += 23;
                                    break;

                                }
                            }




                        }

                }

            }
        }


        var file = new BinaryFile(fileName, BinaryFile.WriteOnly);

        let byte_array = new Uint8Array(array);

        file.write(byte_array.buffer);
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
