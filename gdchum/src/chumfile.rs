use gdnative::*;
use libchum;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct ChumFile {
    data: ByteArray,
    nameid: GodotString,
    typeid: GodotString,
    subtypeid: GodotString
}

#[methods]
impl ChumFile {
    fn _register(builder: &init::ClassBuilder<Self>) {
        builder.add_property("data")
               .with_mut_getter(Self::get_data)
               .with_setter(Self::set_data)
               .done();
        builder.add_property("name")
               .with_mut_getter(Self::get_name)
               .with_setter(Self::set_name)
               .done();
        builder.add_property("type")
               .with_mut_getter(Self::get_type)
               .with_setter(Self::set_type)
               .done();
        builder.add_property("subtype")
               .with_mut_getter(Self::get_subtype)
               .with_setter(Self::set_subtype)
               .done();
    }

    #[export]
    pub fn get_data(&mut self, _owner: Resource) -> ByteArray {
        self.data.new_ref()
    }

    #[export]
    pub fn set_data(&mut self, _owner: Resource, data: ByteArray) {
        self.data = data;
    }

    #[export]
    pub fn get_name(&mut self, _owner: Resource) -> GodotString {
        self.nameid.new_ref()
    }

    #[export]
    pub fn set_name(&mut self, _owner: Resource, value: GodotString) {
        self.nameid = value;
    }

    #[export]
    pub fn get_type(&mut self, _owner: Resource) -> GodotString {
        self.typeid.new_ref()
    }

    #[export]
    pub fn set_type(&mut self, _owner: Resource, value: GodotString) {
        self.typeid = value;
    }

    #[export]
    pub fn get_subtype(&mut self, _owner: Resource) -> GodotString {
        self.subtypeid.new_ref()
    }

    #[export]
    pub fn set_subtype(&mut self, _owner: Resource, value: GodotString) {
        self.subtypeid = value;
    }

    pub fn read_from_chumfile(&mut self, file: &libchum::ChumFile) {
        self.nameid = GodotString::from_str(file.get_name_id());
        self.typeid = GodotString::from_str(file.get_type_id());
        self.subtypeid = GodotString::from_str(file.get_subtype_id());
        let mut data = ByteArray::new();
        let ogdata = file.get_data();
        let len = ogdata.len() as i32;
        data.resize(len);
        for i in 0..len {
            data.set(i, ogdata[i as usize]);
        }
        self.data = data;
    }

    fn _init(_owner: Resource) -> Self {
        ChumFile {
            data: ByteArray::new(),
            nameid: GodotString::new(),
            typeid: GodotString::new(),
            subtypeid: GodotString::new()
        }
    }
}