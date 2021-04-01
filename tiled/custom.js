var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        var m = {
            width: map.width * 15,
            height: map.height * 15,
        };


        var rows = [];
        for (i = map.layerCount - 1; i > 0; i--) {
            var layer = map.layerAt(i);

            if (layer.isTileLayer) {
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.cellAt(x, y).empty == false) {
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

         m.objects = rows;


        var file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(m));
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
