Name:           watermelon
Version:        0.1.0
Release:        1%{?dist}
Summary:        Fresh. Clean. Friendly. Notes App.

License:        MIT
URL:            https://github.com/example/watermelon
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust-packaging
BuildRequires:  gtk4-devel
BuildRequires:  libadwaita-devel

%description
Watermelon is a new Apple Notes–style productivity app built for Fedora Linux.

%prep
%autosetup

%build
%cargo_build

%install
%cargo_install
install -D -m 644 data/com.aashiqumar.watermelon.desktop %{buildroot}%{_datadir}/applications/com.aashiqumar.watermelon.desktop
install -D -m 644 data/com.aashiqumar.watermelon.metainfo.xml %{buildroot}%{_datadir}/metainfo/com.aashiqumar.watermelon.metainfo.xml
install -D -m 644 assets/icon.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/com.aashiqumar.watermelon.svg

%files
%{_bindir}/watermelon
%{_datadir}/applications/com.aashiqumar.watermelon.desktop
%{_datadir}/metainfo/com.aashiqumar.watermelon.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/com.aashiqumar.watermelon.svg
