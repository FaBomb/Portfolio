import { store } from "./connect";
import { fetch_unused, update_metadata, unused_metadata, del_from_url } from "./storage";
import { doc, updateDoc, setDoc, getDoc, deleteDoc,
         collection, addDoc, serverTimestamp, getDocs,
         query, orderBy, limit, startAt, where } from "firebase/firestore";

const adjust_storage = async(used_urls: Array<string>) => {
    const unused_urls = await fetch_unused();
    if (unused_urls.length === 0) {
        used_urls.forEach(used_url => {
            update_metadata(used_url);
        })
    } else {
        unused_urls.forEach(unused_url => {
            used_urls.forEach(used_url => {
                if (unused_url === used_url) {
                    update_metadata(used_url);
                }
            })
        })
    }
    const del_urls = await fetch_unused();

    if (del_urls.length !== 0) {
        const msg = del_urls.length+"個のファイルを削除します。よろしいですか？";
        const value = window.confirm(msg);
        if (value) {
            del_urls.forEach(async unused_url => {
                await del_from_url(unused_url);
            })
        }
    }
}

export const set_content = async(collect: string, article: string) => {
    const json_article = JSON.parse(article);
    const docData = {
        article: json_article,
        created_at: serverTimestamp(),
        updated_at: serverTimestamp(),
    }
    await addDoc(collection(store, collect), docData).then(async () => {
        await adjust_storage(json_article.images).then(() => {
            alert("Successful Posting");
        });
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
    await getDoc(updateRef).then((doc) => {
        const data = doc.data();
        if (data) {
            const images: Array<string> = data.article.images;
            images.forEach(img => {
                unused_metadata(img);
            })
        };
    })
    await updateDoc(updateRef, docData).then(async () => {
        await adjust_storage(json_article.images).then(() => {
            alert("Successful Update");
        });
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
    
    await getDoc(updateRef).then((snapshot) => {
        const data = snapshot.data();
        if (data) {
            data.article.tags.forEach(async (tag: string) => {
                const tagRef = doc(store, "tag", tag);
                let articles: Array<string> = [];
                await getDoc(tagRef).then(tagDoc => {
                    const tagData = tagDoc.data();
                    if (tagData) {
                        articles = tagData.articles;
                    }
                })
                if (json_article.released) {
                    articles.push(id);
                } else {
                    articles = articles.filter((article) => {
                        return article !== id;
                    })
                }
                const updateArticles = {
                    articles: articles,
                }
                console.log(updateArticles)
                await updateDoc(tagRef, updateArticles).catch((error) => {
                    console.error("update_tag_content is error", error);
                });
            })
        };
    })
    await updateDoc(updateRef, docData).then(() => {
        console.log()
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
        articles: [],
    }
    await setDoc(doc(store, "tag", tag), tagData).then((docRef) => {
        alert("Save tag");
    }).catch((error) => {
        console.error("set_tag is error", error);
    });
}

export const del_content =async (collect: string, id: string) => {
    const msg = "コンテンツを削除します。よろしいですか？";
    const value = window.confirm(msg);
    const delRef = doc(store, collect, id);

    if (value) {
        await getDoc(delRef).then((snapshot) => {
            const data = snapshot.data();
            if (data) {
                data.article.images.forEach(async (image: string) => {
                    if (!image.match(/no-img.png/)) {
                        await del_from_url(image);
                    }
                })
            };
        })
        await deleteDoc(delRef).then(() => {
            alert("Successful Article delete");
        }).catch((error) => {
            console.error("del_content is error", error);
        })
    }
}
export const del_category =async (category: string) => {
    await deleteDoc(doc(store, "category", category)).then(() => {
        alert(category+" deleted");
    }).catch((error) => {
        console.error("del_category is error", error);
    })
}
export const del_tag =async (tag: string) => {
    await deleteDoc(doc(store, "tag", tag)).then(() => {
        alert(tag+" deleted");
    }).catch((error) => {
        console.error("del_tag is error", error);
    })
}

export const fetch_categories = async():Promise<string> => {
    const categories:Array<string> = [];
    await getDocs(collection(store, "category")).then((docs) => {
        docs.docs.forEach(doc => {
            categories.push(doc.id);
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
            tags.push(doc.id);
        })
    }).catch((error) => {
        console.error("fetch_tags is error", error);
    });
    const tag_text = {
        tags: tags
    }
    const tags_json = JSON.stringify(tag_text);
    return tags_json;
}

export const fetch_article_size = async(collect:string, is_signed: boolean):Promise<number> => {
    let article_size: number = 0;
    if (is_signed) {
        await getDocs(collection(store, collect)).then((snapshot) => {
            article_size = snapshot.size;
        }).catch((e) => {
            console.error(e);
        })
    } else {
        const articleRef = collection(store, collect);
        const q = query(articleRef, where("article.released", "==", true));
        await getDocs(q).then((snapshot) => {
            article_size = snapshot.size;
        }).catch((e) => {
            console.error(e);
        })
    }
    return article_size;
}
export const fetch_query_size = async(query_name: string, query_content: string):Promise<number> => {
    let query_size: number = 0;
    const queryRef = doc(store, query_name, query_content);
    await getDoc(queryRef).then((queryDoc) => {
        const queryData = queryDoc.data();
        if (queryData) {
            const articles = queryData.articles;
            query_size = articles.length;
        }
    }).catch((e) => {
        console.error(e);
    })
    return query_size;
}

export const fetch_article_contents = async(collect:string, index: number, limit_num: number, is_signed: boolean):Promise<string> => {
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

    const articleRef = collection(store, collect);
    let first;
    if (is_signed) {
        first = query(articleRef, orderBy("created_at", "desc"), limit(start_index + 1));
    } else {
        first = query(articleRef, where("article.released", "==", true),
                orderBy("created_at", "desc"), limit(start_index + 1));
    }

    let lastVisible;
    await getDocs(first).then((documentSnapshots) => {
        lastVisible = documentSnapshots.docs[documentSnapshots.docs.length-1];
    }).catch((error) => {
        console.error("get first docs is error", error);
    });
    if (lastVisible === undefined) {
        const article_contents_json = JSON.stringify("");
        return article_contents_json;
    };

    let q;
    if (is_signed) {
        q = query(articleRef, orderBy("created_at", "desc"), startAt(lastVisible), limit(limit_num) );
    } else {
        q = query(articleRef, where("article.released", "==", true),
            orderBy("created_at", "desc"), startAt(lastVisible), limit(limit_num) );
    }

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
export const fetch_query_contents = async(query_name:string, query_content:string, index: number, limit_num: number):Promise<string> => {
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

    const queryRef = doc(store, query_name, query_content);
    
    await getDoc(queryRef).then(async (queryDoc) => {
            const queryData = queryDoc.data();
            if (queryData) {
                const article_array = queryData.articles.reverse();
                let sliced_articles;
                if (article_array.length < start_index+limit_num) {
                    sliced_articles = article_array.slice(start_index);
                } else {
                    sliced_articles = article_array.slice(start_index, start_index+limit_num);
                }
                for (const article of sliced_articles) {
                    const articleRef = doc(store, "blog", article);
                    await getDoc(articleRef).then((articleDoc) => {
                        const articleData = articleDoc.data({ serverTimestamps: "estimate" });
                        if (articleData) {
                            const year = articleData.updated_at.toDate().getFullYear();
                            const month = articleData.updated_at.toDate().getMonth()+1;
                            const date = articleData.updated_at.toDate().getDate();
                            const article: Article = {
                                id: articleDoc.id,
                                content: articleData.article.content,
                                tags: articleData.article.tags,
                                category: articleData.article.category,
                                released: articleData.article.released,
                                title: articleData.article.title,
                                thumbnail: articleData.article.thumbnail,
                                images: articleData.article.images,
                                updated_at: [year, month, date].join("."),
                            }
                            articles.push(article);
                        }
                    })
                }
            }
    }).catch((error) => {
        console.error("fetch_query_contents is error", error);
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
