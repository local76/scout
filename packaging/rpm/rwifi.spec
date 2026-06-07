Name:           rwifi
Version:        TEMPLATE_VERSION
Release:        1%{?dist}
Summary:        A project template for creating unified local terminal utilities in Rust
License:        MIT
URL:            https://github.com/local76/rWifi
Source0:        %{name}-%{version}.tar.gz

%description
A project template for creating unified local terminal utilities in Rust.

%prep
%setup -q

%build
cargo build --release --locked

%install
rm -rf $RPM_BUILD_ROOT
install -d $RPM_BUILD_ROOT/%{_bindir}
install -d $RPM_BUILD_ROOT/%{_datadir}/applications
install -d $RPM_BUILD_ROOT/%{_datadir}/pixmaps
install -m 755 target/release/rwifi $RPM_BUILD_ROOT/%{_bindir}/rwifi
install -m 644 packaging/desktop/rwifi.desktop $RPM_BUILD_ROOT/%{_datadir}/applications/rwifi.desktop
install -m 644 assets/brand/app_icon.png $RPM_BUILD_ROOT/%{_datadir}/pixmaps/rwifi.png

%files
%{_bindir}/rwifi
%{_datadir}/applications/rwifi.desktop
%{_datadir}/pixmaps/rwifi.png
