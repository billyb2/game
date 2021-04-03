var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        let byte_array = new Uint8Array((map.width * map.height) * 22 + 8);

        byte_array[0] = (map.width * 15 & 0xff000000) >> 24;
        byte_array[1] = (map.width * 15 & 0x00ff0000) >> 16;
        byte_array[2] = (map.width * 15 & 0x0000ff00) >> 8;
        byte_array[3] = (map.width * 15 & 0x000000ff);

        byte_array[4] = (map.height * 15 & 0xff000000) >> 24;
        byte_array[5] = (map.height * 15 & 0x00ff0000) >> 16;
        byte_array[6] = (map.height * 15 & 0x0000ff00) >> 8;
        byte_array[7] = (map.height * 15 & 0x000000ff);

        for (var i = 0; i <= 7; i++) {
            console.log(byte_array[i]);
        }

        let index = 8;


        var rows = [];
        for (i = map.layerCount  - 1; i >= 0; i--) {
            var layer = map.layerAt(i);

            if (layer.isTileLayer) {
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.tileAt(x, y) != null) {
                            // x coor
                            byte_array[index + 0] = (x * 15 & 0xff000000) >> 24;
                            byte_array[index + 1] = (x * 15 & 0x00ff0000) >> 16;
                            byte_array[index + 2] = (x * 15 & 0x0000ff00) >> 8;
                            byte_array[index + 3] = (x * 15 & 0x000000ff);

                            //y coor
                            byte_array[index + 4] = (y * 15 & 0xff000000) >> 24;
                            byte_array[index + 5] = (y * 15 & 0x00ff0000) >> 16;
                            byte_array[index + 6] = (y * 15 & 0x0000ff00) >> 8;
                            byte_array[index + 7] = (y * 15 & 0x000000ff);

                            //width
                            byte_array[index + 8] = (15 & 0xff000000) >> 24;
                            byte_array[index + 9] = (15 & 0x00ff0000) >> 16;
                            byte_array[index + 10] = (15 & 0x0000ff00) >> 8;
                            byte_array[index + 11] = (15 & 0x000000ff);

                            //height
                            byte_array[index + 12] = (15 & 0xff000000) >> 24;
                            byte_array[index + 13] = (15 & 0x00ff0000) >> 16;
                            byte_array[index + 14] = (15 & 0x0000ff00) >> 8;
                            byte_array[index + 15] = (15 & 0x000000ff);

                            if (layer.tileAt(x, y).property("player_spawn") == true) {
                                byte_array[index + 16] = 255;

                            } else {
                                byte_array[index + 16] = 0;

                            }

                            if (layer.tileAt(x, y).property("player_collidable") == true) {
                                byte_array[index + 17] = 255;

                            } else {
                                byte_array[index + 17] = 0;

                            }

                            byte_array[index + 18] = layer.tileAt(x, y).property("red");
                            byte_array[index + 19] = layer.tileAt(x, y).property("green");
                            byte_array[index + 20] = layer.tileAt(x, y).property("blue");
                            byte_array[index + 21] = layer.tileAt(x, y).property("alpha");
                            byte_array[index + 22] = 0;

                            for (var j = index; j < index + 22; j ++) {
                                if (byte_array[j] != 0) {
                                    index += 23;
                                    break;

                                }

                            }

                            rows.push({
                                data: {
                                    x: x * 15,
                                    y: y * 15,
                                    w: 15,
                                    h: 15
                                },
                                player_spawn: layer.tileAt(x, y).property("player_spawn"),
                                player_collidable: layer.tileAt(x, y).property("player_collidable"),
                                color: [layer.tileAt(x, y).property("red") / 255.0,
                                            layer.tileAt(x, y).property("green") / 255.0,
                                            layer.tileAt(x, y).property("blue") / 255.0,
                                            layer.tileAt(x, y).property("alpha") / 255.0]
                                //health: 100


                            });

                        }

                }

            }
        }

        var file = new BinaryFile(fileName, BinaryFile.WriteOnly);

        file.write(byte_array.buffer);
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
