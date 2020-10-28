const can = require('socketcan')
const assert = require('assert')
var argv = require('minimist')(process.argv.slice(2));

assert(argv.def, "argument `--def=foo.kcd` required")

const kcd = can.parseNetworkDescription(argv.def);
const channel = can.createRawChannel('vcan0')
var db = new can.DatabaseService(channel, kcd.buses['Example']);
channel.start()

db.messages["Foo"].signals["Voltage"].update(2);
db.messages["Foo"].signals["Current"].update(2);
db.send("Foo");

db.messages["Foo"].signals["Voltage"].update(42.42);
db.messages["Foo"].signals["Current"].update(13.37);
db.send("Foo");

db.messages["Bar"].signals["One"].update(2);
db.messages["Bar"].signals["Two"].update(2);
db.messages["Bar"].signals["Three"].update(2);
db.messages["Bar"].signals["Four"].update(2);
db.send("Bar");

db.messages["Bar"].signals["One"].update(1);
db.messages["Bar"].signals["Two"].update(2);
db.messages["Bar"].signals["Three"].update(3);
db.messages["Bar"].signals["Four"].update(1);
db.send("Bar");

setTimeout(() => {
    channel.stop();
}, 200)
