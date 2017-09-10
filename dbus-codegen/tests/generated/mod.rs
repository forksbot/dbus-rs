// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusProperties for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        Ok(value)
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> = try!(i.read());
        Ok(properties)
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Set".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
            i.append(value);
        }));
        try!(m.as_result());
        Ok(())
    }
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusProperties<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let property_name: &str = try!(i.read());
        let d = fclone(minfo);
        let value = try!(d.get(interface_name, property_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let d = fclone(minfo);
        let properties = try!(d.get_all(interface_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let property_name: &str = try!(i.read());
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        let d = fclone(minfo);
        try!(d.set(interface_name, property_name, value));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);
    i
}

pub fn org_freedesktop_dbus_properties_properties_changed_emit<C: ::std::ops::Deref<Target=dbus::Connection>>(conn: &dbus::ConnPath<C>, interface_name: String, changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, invalidated_properties: Vec<String>) -> Result<(), dbus::Error> {
    conn.signal_with_args(&"org.freedesktop.DBus.Properties".into(), &"PropertiesChanged".into(), move |msg| {
         let mut i = arg::IterAppend::new(msg);
         i.append(interface_name);
         i.append(changed_properties);
         i.append(invalidated_properties);
    }).map(|_| ())
}

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusIntrospectable for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let xml_data: String = try!(i.read());
        Ok(xml_data)
    }
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml_data = try!(d.introspect());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml_data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml_data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusPeer {
    type Err;
    fn ping(&self) -> Result<(), Self::Err>;
    fn get_machine_id(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusPeer for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn ping(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"Ping".into(), |_| {
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_machine_id(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"GetMachineId".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let machine_uuid: String = try!(i.read());
        Ok(machine_uuid)
    }
}

pub fn org_freedesktop_dbus_peer_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusPeer<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Peer", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        try!(d.ping());
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Ping", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let machine_uuid = try!(d.get_machine_id());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(machine_uuid);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMachineId", Default::default(), h);
    let m = m.out_arg(("machine_uuid", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopPolicyKit1Authority {
    type Err;
    fn enumerate_actions(&self, locale: &str) -> Result<Vec<(String, String, String, String, String, String, u32, u32, u32, ::std::collections::HashMap<String, String>)>, Self::Err>;
    fn check_authorization(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), action_id: &str, details: ::std::collections::HashMap<&str, &str>, flags: u32, cancellation_id: &str) -> Result<(bool, bool, ::std::collections::HashMap<String, String>), Self::Err>;
    fn cancel_check_authorization(&self, cancellation_id: &str) -> Result<(), Self::Err>;
    fn register_authentication_agent(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), locale: &str, object_path: &str) -> Result<(), Self::Err>;
    fn register_authentication_agent_with_options(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), locale: &str, object_path: &str, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn unregister_authentication_agent(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), object_path: &str) -> Result<(), Self::Err>;
    fn authentication_agent_response(&self, cookie: &str, identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err>;
    fn authentication_agent_response2(&self, uid: u32, cookie: &str, identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err>;
    fn enumerate_temporary_authorizations(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<Vec<(String, String, (String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>), u64, u64)>, Self::Err>;
    fn revoke_temporary_authorizations(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err>;
    fn revoke_temporary_authorization_by_id(&self, id: &str) -> Result<(), Self::Err>;
    fn get_backend_name(&self) -> Result<String, Self::Err>;
    fn get_backend_version(&self) -> Result<String, Self::Err>;
    fn get_backend_features(&self) -> Result<u32, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopPolicyKit1Authority for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn enumerate_actions(&self, locale: &str) -> Result<Vec<(String, String, String, String, String, String, u32, u32, u32, ::std::collections::HashMap<String, String>)>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"EnumerateActions".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(locale);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let action_descriptions: Vec<(String, String, String, String, String, String, u32, u32, u32, ::std::collections::HashMap<String, String>)> = try!(i.read());
        Ok(action_descriptions)
    }

    fn check_authorization(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), action_id: &str, details: ::std::collections::HashMap<&str, &str>, flags: u32, cancellation_id: &str) -> Result<(bool, bool, ::std::collections::HashMap<String, String>), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"CheckAuthorization".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
            i.append(action_id);
            i.append(details);
            i.append(flags);
            i.append(cancellation_id);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let result: (bool, bool, ::std::collections::HashMap<String, String>) = try!(i.read());
        Ok(result)
    }

    fn cancel_check_authorization(&self, cancellation_id: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"CancelCheckAuthorization".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(cancellation_id);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn register_authentication_agent(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), locale: &str, object_path: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"RegisterAuthenticationAgent".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
            i.append(locale);
            i.append(object_path);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn register_authentication_agent_with_options(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), locale: &str, object_path: &str, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"RegisterAuthenticationAgentWithOptions".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
            i.append(locale);
            i.append(object_path);
            i.append(options);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn unregister_authentication_agent(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>), object_path: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"UnregisterAuthenticationAgent".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
            i.append(object_path);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn authentication_agent_response(&self, cookie: &str, identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"AuthenticationAgentResponse".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(cookie);
            i.append(identity);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn authentication_agent_response2(&self, uid: u32, cookie: &str, identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"AuthenticationAgentResponse2".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(uid);
            i.append(cookie);
            i.append(identity);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn enumerate_temporary_authorizations(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<Vec<(String, String, (String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>), u64, u64)>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"EnumerateTemporaryAuthorizations".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let temporary_authorizations: Vec<(String, String, (String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>), u64, u64)> = try!(i.read());
        Ok(temporary_authorizations)
    }

    fn revoke_temporary_authorizations(&self, subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>)) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"RevokeTemporaryAuthorizations".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(subject);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn revoke_temporary_authorization_by_id(&self, id: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"RevokeTemporaryAuthorizationById".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(id);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_backend_name(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), move |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append("org.freedesktop.PolicyKit1.Authority");
            i.append("BackendName");
        }));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_backend_version(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), move |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append("org.freedesktop.PolicyKit1.Authority");
            i.append("BackendVersion");
        }));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_backend_features(&self) -> Result<u32, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), move |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append("org.freedesktop.PolicyKit1.Authority");
            i.append("BackendFeatures");
        }));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }
}

pub fn org_freedesktop_policy_kit1_authority_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgFreedesktopPolicyKit1Authority<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.PolicyKit1.Authority", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let locale: &str = try!(i.read());
        let d = fclone(minfo);
        let action_descriptions = try!(d.enumerate_actions(locale));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(action_descriptions);
        Ok(vec!(rm))
    };
    let m = factory.method("EnumerateActions", Default::default(), h);
    let m = m.in_arg(("locale", "s"));
    let m = m.out_arg(("action_descriptions", "a(ssssssuuua{ss})"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let action_id: &str = try!(i.read());
        let details: ::std::collections::HashMap<&str, &str> = try!(i.read());
        let flags: u32 = try!(i.read());
        let cancellation_id: &str = try!(i.read());
        let d = fclone(minfo);
        let result = try!(d.check_authorization(subject, action_id, details, flags, cancellation_id));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(result);
        Ok(vec!(rm))
    };
    let m = factory.method("CheckAuthorization", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let m = m.in_arg(("action_id", "s"));
    let m = m.in_arg(("details", "a{ss}"));
    let m = m.in_arg(("flags", "u"));
    let m = m.in_arg(("cancellation_id", "s"));
    let m = m.out_arg(("result", "(bba{ss})"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let cancellation_id: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.cancel_check_authorization(cancellation_id));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("CancelCheckAuthorization", Default::default(), h);
    let m = m.in_arg(("cancellation_id", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let locale: &str = try!(i.read());
        let object_path: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.register_authentication_agent(subject, locale, object_path));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("RegisterAuthenticationAgent", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let m = m.in_arg(("locale", "s"));
    let m = m.in_arg(("object_path", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let locale: &str = try!(i.read());
        let object_path: &str = try!(i.read());
        let options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>> = try!(i.read());
        let d = fclone(minfo);
        try!(d.register_authentication_agent_with_options(subject, locale, object_path, options));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("RegisterAuthenticationAgentWithOptions", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let m = m.in_arg(("locale", "s"));
    let m = m.in_arg(("object_path", "s"));
    let m = m.in_arg(("options", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let object_path: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.unregister_authentication_agent(subject, object_path));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("UnregisterAuthenticationAgent", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let m = m.in_arg(("object_path", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let cookie: &str = try!(i.read());
        let identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let d = fclone(minfo);
        try!(d.authentication_agent_response(cookie, identity));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("AuthenticationAgentResponse", Default::default(), h);
    let m = m.in_arg(("cookie", "s"));
    let m = m.in_arg(("identity", "(sa{sv})"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let uid: u32 = try!(i.read());
        let cookie: &str = try!(i.read());
        let identity: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let d = fclone(minfo);
        try!(d.authentication_agent_response2(uid, cookie, identity));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("AuthenticationAgentResponse2", Default::default(), h);
    let m = m.in_arg(("uid", "u"));
    let m = m.in_arg(("cookie", "s"));
    let m = m.in_arg(("identity", "(sa{sv})"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let d = fclone(minfo);
        let temporary_authorizations = try!(d.enumerate_temporary_authorizations(subject));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(temporary_authorizations);
        Ok(vec!(rm))
    };
    let m = factory.method("EnumerateTemporaryAuthorizations", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let m = m.out_arg(("temporary_authorizations", "a(ss(sa{sv})tt)"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let subject: (&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
        let d = fclone(minfo);
        try!(d.revoke_temporary_authorizations(subject));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("RevokeTemporaryAuthorizations", Default::default(), h);
    let m = m.in_arg(("subject", "(sa{sv})"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let id: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.revoke_temporary_authorization_by_id(id));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("RevokeTemporaryAuthorizationById", Default::default(), h);
    let m = m.in_arg(("id", "s"));
    let i = i.add_m(m);

    let p = factory.property::<&str, _>("BackendName", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_backend_name()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("BackendVersion", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_backend_version()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("BackendFeatures", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_backend_features()));
        Ok(())
    });
    let i = i.add_p(p);
    i
}

pub fn org_freedesktop_policy_kit1_authority_changed_emit<C: ::std::ops::Deref<Target=dbus::Connection>>(conn: &dbus::ConnPath<C>) -> Result<(), dbus::Error> {
    conn.signal_with_args(&"org.freedesktop.PolicyKit1.Authority".into(), &"Changed".into(),  |_| {}).map(|_| ())
}