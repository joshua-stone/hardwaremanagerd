#!/usr/bin/env python3

# This is a basic device lookup program for finding the hardware model information, namely GPUs

# Example output:
# $ ./getdeviceinfo.py
# Normal order lookup:
# Ellesmere [Radeon RX 470/480/570/570X/580/580X/590]
# Reverse order lookup:
# Radeon RX 580

# pip3 install --user pyudev libpci

from pyudev import Context
from libpci import LibPCI

context = Context()
pci = LibPCI()

for device in context.list_devices(subsystem='pci',ID_PCI_CLASS_FROM_DATABASE='Display controller'):
    vendor_id, device_id = [int(i, 16) for i in device.properties['PCI_ID'].split(':')]
    subvendor_id, subdevice_id = [int(i, 16) for i in device.properties['PCI_SUBSYS_ID'].split(':')]

    model = pci.lookup_subsystem_device_name(vendor_id=vendor_id, device_id=device_id, 
                                             subvendor_id=subvendor_id, subdevice_id=subvendor_id)

    model_reverse = pci.lookup_subsystem_device_name(vendor_id=subvendor_id, device_id=subdevice_id,
                                                     subvendor_id=vendor_id, subdevice_id=vendor_id)

    print('Normal order lookup:')
    print(model)
    # Since some devices have a backwards lookup order, so print both versions to cover edge cases
    print('Reverse order lookup:')
    print(model_reverse)

