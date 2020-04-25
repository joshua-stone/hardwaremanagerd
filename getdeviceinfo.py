#!/usr/bin/env python3

# To setup, run:
# $ pip3 install --user pyudev libpci

# This is a basic device lookup program for finding hardware model information, namely GPUs

# Example output:
# $ ./getdeviceinfo.py
# Device #0
# Normal order lookup:
# [1002:67df 1682:c580]
# vendor=Advanced Micro Devices, Inc. [AMD/ATI]
# model=Ellesmere [Radeon RX 470/480/570/570X/580/580X/590]
#
# Reverse order lookup:
# [1002:67df 1682:c580]
# vendor=XFX Pine Group Inc.
# model=Radeon RX 580

from pyudev import Context
from libpci import LibPCI

context = Context()
pci = LibPCI()

for number, device in enumerate(context.list_devices(subsystem='pci',ID_PCI_CLASS_FROM_DATABASE='Display controller')):
    vendor_id,    device_id    = [int(i, 16) for i in device.properties['PCI_ID'].split(':')]
    subvendor_id, subdevice_id = [int(i, 16) for i in device.properties['PCI_SUBSYS_ID'].split(':')]

    vendor = pci.lookup_vendor_name(vendor_id)
    model  = pci.lookup_subsystem_device_name(vendor_id=vendor_id,       device_id=device_id,
                                              subvendor_id=subvendor_id, subdevice_id=subdevice_id)

    vendor_reverse = pci.lookup_vendor_name(subvendor_id)
    model_reverse  = pci.lookup_subsystem_device_name(vendor_id=subvendor_id, device_id=subdevice_id,
                                                      subvendor_id=vendor_id, subdevice_id=device_id)

    print(f'Device #{number}')
    print('Normal order lookup:')
    print(f'[{vendor_id:02x}:{device_id:02x} {subvendor_id:02x}:{subdevice_id:02x}]')
    print(f'vendor = {vendor}')
    print(f'model = {model}')
    print()
    # Some devices have a backwards lookup order, so print both versions to cover edge cases
    print('Reverse order lookup:')
    print(f'[{vendor_id:02x}:{device_id:02x} {subvendor_id:02x}:{subdevice_id:02x}]')
    print(f'vendor = {vendor_reverse}')
    print(f'model = {model_reverse}')
    print()

