import pymodbus
import serial
import logging
from pymodbus.pdu import ModbusRequest
from pymodbus.client.sync import (
    ModbusSerialClient as ModbusClient,
)  # initialize a serial RTU client instance
from pymodbus.transaction import ModbusRtuFramer


logging.basicConfig()
log = logging.getLogger()
log.setLevel(logging.DEBUG)

# count= the number of registers to read
# unit= the slave unit this request is targeting
# address= the starting address to read from

client = ModbusClient(
    method="rtu", port="COM6", stopbits=1, bytesize=8, parity="E", baudrate=19200
)

# Connect to the serial modbus server
connection = client.connect()
print(connection)

# Starting add, num of reg to read, slave unit.
result = client.read_holding_registers(0x00, 2, unit=0xFF)

print(result)

# Closes the underlying socket connection
client.close()