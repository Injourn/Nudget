interface CategoryModel{
    name: string;
    childCategories: CategoryModel[];
}

export default CategoryModel;