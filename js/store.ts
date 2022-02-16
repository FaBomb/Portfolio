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

export const del_category =async (category: string) => {
    await deleteDoc(doc(store, "category", category)).then(() => {
        console.log(category+"を削除しました")
    }).catch((error) => {
        console.error("del_category is error", error);
    })
}
export const del_tag =async (tag: string) => {
    await deleteDoc(doc(store, "tag", tag)).then(() => {
        console.log(tag+"を削除しました")
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

export const fetch_article_id = async(collect:string, lastId: string):Promise<string> => {
    const limit_num: number = 2;
    const article_ids:Array<string> = [];

    const lastDocRef = doc(store, collect, lastId);
    const q = query(collection(store, collect), orderBy("created_at", "desc"),
                    endAt(), limit(limit_num) );

    await getDocs(q).then((snapshot) => {
        console.log(snapshot);
        snapshot.docs.forEach(doc => {
            article_ids.push(doc.id);
        })
    }).catch((error) => {
        console.error("fetch_article_id is error", error);
    });
    const article_ids_json = JSON.stringify(article_ids);
    return article_ids_json;
}

export const fetch_article_contents = async(collect:string, id: string):Promise<string> => {
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
