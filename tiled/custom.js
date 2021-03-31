var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        var m = {
            width: map.width * 5,
            height: map.height * 5,
        };

            var layer = map.layerAt(0);

            if (layer.isTileLayer) {
                var rows = [];
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.cellAt(x, y).empty == false) {
                            rows.push({
                                data: {
                                    x: x * 5,
                                    y: y * 5,
                                    w: 5,
                                    h: 5
                                },
                                color: [255, 255, 255, 255],
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
