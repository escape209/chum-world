extends Tree

signal file_selected(file)

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const ChumFile := preload("res://gdchum/ChumFile.gdns")

class HierarchyItem:
	var item: TreeItem
	var dict: Dictionary
	func _init(p_item: TreeItem):
		self.item = p_item
		self.dict = {}

var hierarchy: HierarchyItem
var tree_root: TreeItem
var archive = null
var archive_files = []

func get_type_icon(typename):
	if typename in MeshData.TYPE_ICONS:
		return MeshData.TYPE_ICONS[typename]
	return MeshData.ICON_UNKNOWN

func split_fname(name: String) -> PoolStringArray:
	return name.split(">", false)

func get_item_parent(name: String) -> TreeItem:
	var split := split_fname(name)
	var current := hierarchy
	var i := 0
	while i < split.size() - 1:
		var s := split[i]
		if s in current.dict:
			current = current.dict[s]
		else:
			var item := create_item(current.item)
			item.set_selectable(0, false)
			item.set_selectable(1, false)
			item.set_selectable(2, false)
			item.set_custom_color(0, Color.darkgray)
			item.set_text(0, s)
			item.set_meta("category", true)
			var newh := HierarchyItem.new(item)
			current.dict[s] = newh
			current = newh
		i += 1
	return current.item

func _sort_file(a, b):
	if a.name != b.name:
		var acount = a.name.count(">")
		var bcount = b.name.count(">")
		if acount != bcount:
			return bcount < acount
		if a.type != b.type:
			return a.type < b.type
		return a.name < b.name
	elif a.type != b.type:
		return a.type < b.type
	else:
		return a.subtype < b.subtype

var prev_search := ""
func do_search(text: String, selected_file):
	prev_search = text
	prints("SEARCH", text, archive)
	text = text.to_lower()
	clear()
	tree_root = create_item()
	hierarchy = HierarchyItem.new(tree_root)
	hide_root = true
	if archive == null:
		return
	for file in archive_files:
		var name := file.name as String
		var type := file.type as String
		var subtype := file.subtype as String
		if text != "" and not text in name.to_lower()\
					  and not text in type.to_lower()\
					  and not text in subtype.to_lower()\
					  and not file == selected_file:
			continue
		var item := create_item(get_item_parent(name))
		item.set_icon(0, get_type_icon(type))
		item.set_text(0, split_fname(name)[-1])
		item.set_text(1, type)
		item.set_text(2, subtype)
		item.set_custom_color(0, Color.white)
		item.set_meta("file", file)
		item.set_meta("category", false)
		if file == selected_file:
			_do_skip_select = true
			item.select(0)
			_do_skip_select = false

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	archive_files.sort_custom(self, "_sort_file")
	do_search(prev_search, null)

var _do_skip_select := false
func _on_Tree_item_selected():
	if not _do_skip_select:
		emit_signal("file_selected", get_selected().get_meta("file"))

func _ready():
	columns = 3
	set_column_titles_visible(true)
	set_column_title(0, "Name")
	set_column_title(1, "Type")
	set_column_title(2, "Subtype")
	set_column_min_width(0, 8)
	set_column_min_width(1, 4)
	set_column_min_width(2, 4)

func _on_LineEdit_text_changed(new_text: String):
	var current_file = null
	if get_selected() != null:
		current_file = get_selected().get_meta("file")
	do_search(new_text, current_file)

func set_selected(new_file):
	do_search(prev_search, new_file)

var _tree_menu_item: TreeItem = null
func _gui_input(event):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_RIGHT and event.pressed:
			_tree_menu_item = get_item_at_position(event.position)
			if _tree_menu_item != null and _tree_menu_item.get_meta("category") == false:
				var pos = get_global_mouse_position()
				$TreeMenu.popup(Rect2(pos - Vector2.ONE*4, Vector2.ONE))

const TREE_MENU_COPY_ID := 0
const TREE_MENU_COPY_PATH := 1

func _on_TreeMenu_id_pressed(id):
	if id == TREE_MENU_COPY_ID:
		var file_id = _tree_menu_item.get_meta("file").get_hash_id()
		OS.clipboard = str(file_id)
	elif id == TREE_MENU_COPY_PATH:
		var path = _tree_menu_item.get_meta("file").name
		OS.clipboard = path
