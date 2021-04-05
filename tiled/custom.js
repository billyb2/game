var CRC32;!function(n){"undefined"==typeof DO_NOT_EXPORT_CRC?"object"==typeof exports?n(exports):"function"==typeof define&&define.amd?define(function(){var r={};return n(r),r}):n(CRC32={}):n(CRC32={})}(function(r){r.version="1.2.0";var a=function(){for(var r=0,n=new Array(256),e=0;256!=e;++e)r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=e)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1,n[e]=r;return"undefined"!=typeof Int32Array?new Int32Array(n):n}();r.table=a,r.bstr=function(r,n){for(var e=-1^n,t=r.length-1,o=0;o<t;)e=(e=e>>>8^a[255&(e^r.charCodeAt(o++))])>>>8^a[255&(e^r.charCodeAt(o++))];return o===t&&(e=e>>>8^a[255&(e^r.charCodeAt(o))]),-1^e},r.buf=function(r,n){if(1e4<r.length)return function(r,n){for(var e=-1^n,t=r.length-7,o=0;o<t;)e=(e=(e=(e=(e=(e=(e=(e=e>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])];for(;o<7+t;)e=e>>>8^a[255&(e^r[o++])];return-1^e}(r,n);for(var e=-1^n,t=r.length-3,o=0;o<t;)e=(e=(e=(e=e>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])];for(;o<3+t;)e=e>>>8^a[255&(e^r[o++])];return-1^e},r.str=function(r,n){for(var e,t,o=-1^n,f=0,u=r.length;f<u;)o=(e=r.charCodeAt(f++))<128?o>>>8^a[255&(o^e)]:e<2048?(o=o>>>8^a[255&(o^(192|e>>6&31))])>>>8^a[255&(o^(128|63&e))]:55296<=e&&e<57344?(e=64+(1023&e),t=1023&r.charCodeAt(f++),(o=(o=(o=o>>>8^a[255&(o^(240|e>>8&7))])>>>8^a[255&(o^(128|e>>2&63))])>>>8^a[255&(o^(128|t>>6&15|(3&e)<<4))])>>>8^a[255&(o^(128|63&t))]):(o=(o=o>>>8^a[255&(o^(224|e>>12&15))])>>>8^a[255&(o^(128|e>>6&63))])>>>8^a[255&(o^(128|63&e))];return-1^o}});


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

        // An entirely null map object signifies the start of the crc32 hash
        for (var i = 1; i < 22; i ++) {
            array.push(0);

        }

        //array.push(1);


        var file = new BinaryFile(fileName, BinaryFile.WriteOnly);

        let tmp_byte_array = new Uint8Array(array);

        let crc32 = CRC32.buf(tmp_byte_array);

        array.push((crc32 & 0xff000000) >> 24);
        array.push((crc32 & 0x00ff0000) >> 16);
        array.push((crc32 & 0x0000ff00) >> 8);
        array.push(crc32 & 0x000000ff);

        let byte_array = new Uint8Array(array);

        file.write(byte_array.buffer);
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
