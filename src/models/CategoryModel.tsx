interface CategoryModel{
    name: string;
    parent_category?: CategoryModel;
}

export default CategoryModel;