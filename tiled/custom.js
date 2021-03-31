var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        var m = {
            width: map.width * 15,
            height: map.height * 15,
        };

            var layer = map.layerAt(0);

            if (layer.isTileLayer) {
                var rows = [];
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.cellAt(x, y).empty == false) {
                            let color = [255.0, 255.0, 255.0, 255.0];

                            rows.push({
                                data: {
                                    x: x * 15,
                                    y: y * 15,
                                    w: 15,
                                    h: 15
                                },
                                color: [layer.tileAt(x, y).property("red"), layer.tileAt(x, y).property("green"), layer.tileAt(x, y).property("blue"), 255.0],
                                //health: 100


                            });

                        }

                }

                m.objects = rows;
            }


        var file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(m));
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
