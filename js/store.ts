import { store } from "./connect";
import { fetch_unused, update_metadata, del_from_url } from "./storage";
import { doc, updateDoc, setDoc, getDoc, deleteDoc,
         collection, addDoc, serverTimestamp, getDocs,
         query, orderBy, limit, startAt, endAt } from "firebase/firestore";

const adjust_storage = async(used_urls: Array<string>) => {
    const unused_urls = await fetch_unused();
    unused_urls.forEach(unused_url => {
        used_urls.forEach(used_url => {
            if (unused_url === used_url) {
                update_metadata(used_url);
            } else {
                del_from_url(unused_url);
            }
        })
    })
}

export const set_content = async(collect: string, article: string) => {
    const json_article = JSON.parse(article);
    const docData = {
        article: json_article,
        created_at: serverTimestamp(),
        updated_at: serverTimestamp(),
    }
    await addDoc(collection(store, collect), docData).then(() => {
        adjust_storage(json_article.images);
        alert("Successful Posting");
    }).catch((error) => {
        console.error("set_content is error", error);
    });
}

export const update_content = async(collect: string, article: string, id: string) => {
    const json_article = JSON.parse(article);
    const docData = {
        article: json_article,
        updated_at: serverTimestamp(),
    }
    const updateRef = doc(store, collect, id);
    await updateDoc (updateRef, docData).then(() => {
        adjust_storage(json_article.images);
        alert("Successful Update");
    }).catch((error) => {
        console.error("update_content is error", error);
    });
}
export const update_released = async(collect: string, article: string, id: string) => {
    const json_article = JSON.parse(article);
    json_article.released = !json_article.released;
    const docData = {
        article: json_article,
    }
    const updateRef = doc(store, collect, id);
    await updateDoc (updateRef, docData).then(() => {
        if (json_article.released) {
            alert("Public");
        } else {
            alert("Private");
        }
    }).catch((error) => {
        console.error("update_content is error", error);
    });
}

export const set_category = async(category: string) => {
    const categoryData = {
        category: category,
    }
    await setDoc(doc(store, "category", category), categoryData).then(() => {
        alert("Save Category");
    }).catch((error) => {
        console.error("set_category is error", error);
    });
}
export const set_tag = async(tag: string) => {
    const tagData = {
        tag: tag,
    }
    await setDoc(doc(store, "tag", tag), tagData).then((docRef) => {
        alert("Save tag");
    }).catch((error) => {
        console.error("set_tag is error", error);
    });
}

export const del_content =async (collect: string, id: string) => {
    const delRef = doc(store, collect, id);
    await deleteDoc(delRef).then(() => {
        alert("Successful Article delete");
    }).catch((error) => {
        console.error("del_content is error", error);
    })
}
export const del_category =async (category: string) => {
    await deleteDoc(doc(store, "category", category)).then(() => {
        console.log(category+" deleted")
    }).catch((error) => {
        console.error("del_category is error", error);
    })
}
export const del_tag =async (tag: string) => {
    await deleteDoc(doc(store, "tag", tag)).then(() => {
        console.log(tag+" deleted")
    }).catch((error) => {
        console.error("del_tag is error", error);
    })
}

export const fetch_categories = async():Promise<string> => {
    const categories:Array<string> = [];
    await getDocs(collection(store, "category")).then((docs) => {
        docs.docs.forEach(doc => {
            categories.push(doc.data().category);
        })
    }).catch((error) => {
        console.error("fetch_categories is error", error);
    });
    const categories_json = JSON.stringify(categories);
    return categories_json;
}

export const fetch_tags = async():Promise<string> => {
    const tags:Array<string> = [];
    await getDocs(collection(store, "tag")).then((docs) => {
        docs.docs.forEach(doc => {
            tags.push(doc.data().tag);
        })
    }).catch((error) => {
        console.error("fetch_tags is error", error);
    });
    const tags_json = JSON.stringify(tags);
    return tags_json;
}

export const fetch_article_size = async(collect:string):Promise<number> => {
    let article_size: number = 0; 
    await getDocs(collection(store, collect)).then((snapshot) => {
        article_size = snapshot.size;
    }).catch((e) => {
        console.error(e);
    })
    return article_size;
}

export const fetch_article_contents = async(collect:string, index: number, limit_num: number):Promise<string> => {
    const start_index: number = limit_num * (index - 1);
    const articles: Array<Article> = [];
    interface Article{
        id: string,
        content: string,
        tags: Array<string>,
        category: string,
        released: boolean,
        title: string,
        thumbnail: string,
        images: Array<string>,
        updated_at: string,
    }

    const first = query(collection(store, collect), orderBy("created_at", "desc"),
                        limit(start_index + 1));
    const documentSnapshots = await getDocs(first);
    const lastVisible = documentSnapshots.docs[documentSnapshots.docs.length-1];

    const q = query(collection(store, collect), orderBy("created_at", "desc"),
                    startAt(lastVisible), limit(limit_num) );

    await getDocs(q).then((snapshot) => {
        snapshot.docs.forEach(doc => {
            const data = doc.data({ serverTimestamps: "estimate" });
            if (data) {
                const year = data.updated_at.toDate().getFullYear();
                const month = data.updated_at.toDate().getMonth()+1;
                const date = data.updated_at.toDate().getDate();
                const article: Article = {
                    id: doc.id,
                    content: data.article.content,
                    tags: data.article.tags,
                    category: data.article.category,
                    released: data.article.released,
                    title: data.article.title,
                    thumbnail: data.article.thumbnail,
                    images: data.article.images,
                    updated_at: [year, month, date].join("."),
                }
                articles.push(article);
            }

        })
    }).catch((error) => {
        console.error("fetch_article_contents is error", error);
    });
    const article_contents_json = JSON.stringify(articles);
    return article_contents_json;
}
export const fetch_article_content_from_id = async(collect:string, id: string):Promise<string> => {
    let article;
    await getDoc(doc(store, collect, id)).then((snapshot) => {
        const data = snapshot.data({ serverTimestamps: "estimate" });
        if (data) {
            article = data.article;
            const year = data.updated_at.toDate().getFullYear();
            const month = data.updated_at.toDate().getMonth()+1;
            const date = data.updated_at.toDate().getDate();
            article.updated_at = [year, month, date].join(".");
        }
    }).catch((error) => {
        console.error("fetch_article_contents is error", error);
    });
    const article_ids_json = JSON.stringify(article);
    return article_ids_json;
}

export const fetch_all_article_content_from_id = async(collect:string, id: string):Promise<string> => {
    let article;
    await getDoc(doc(store, collect, id)).then((snapshot) => {
        const data = snapshot.data({ serverTimestamps: "estimate" });
        if (data) {
            article = {
                content: data.article.content,
                tags: data.article.tags,
                category: data.article.category,
                title: data.article.title,
                thumbnail: data.article.thumbnail,
            }
        }
    }).catch((error) => {
        console.error("fetch_article_contents is error", error);
    });

    const article_ids_json = JSON.stringify(article);
    return article_ids_json;
}
