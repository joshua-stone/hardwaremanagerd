Name         : hardwaremanagerd
Version      : 0.1
Release      : 1%{?dist}
License      : MIT
URL          : https://github.com/joshua-stone/hardwaremanagerd
Summary      : Hardware Manager service
BuildRequires: dbus-devel,systemd-devel,gcc

%description
A daemon for getting hardware information and setting hardware profiles

%install
%define _configdir /etc/hardwaremonitord

cargo build --release
mkdir -p %{buildroot}%{_sbindir}
mkdir -p %{buildroot}%{_unitdir}
mkdir -p %{buildroot}%{_configdir}/profiles/pci

install -p -m 755 target/release/hardwaremanagerd                 %{buildroot}%{_sbindir}
install -p -m 644 runtime/%{_unitdir}/hardwaremanagerd.service    %{buildroot}%{_unitdir}
install -p -m 644 runtime/%{_configdir}/profiles/pci/amdgpu.conf  %{buildroot}%{_configdir}/profiles/pci/amdgpu.conf
install -p -m 644 runtime/%{_configdir}/profiles/pci/nvidia.conf  %{buildroot}%{_configdir}/profiles/pci/nvidia.conf

%files
%{_sbindir}/hardwaremanagerd
%{_unitdir}/hardwaremanagerd.service
%{_sysconfdir}/hardwaremonitord/profiles/pci/amdgpu.conf
%{_sysconfdir}/hardwaremonitord/profiles/pci/nvidia.conf
