import app from '/js/site.min.js'
const d = app.d, df = d.domfn
const {div, span} = df

const wwLauncher = d.query('.ww-launcher')

const {
    wwView,
    titleInput,
    writingPad,
    tagInput,
    pushWritBtn,
    clearEditorBtn,
    writSelector,
    isPublicCheckbox,
    isCommentableCheckbox,
    writList
} = d.h/* html */`
<article class="writ-writer-view" ref="wwView">
    <section class="writer">
        <div>
            <input type="text" name="post-title" title="writ title" id="title-input" placeholder="writ title" autocomplete="off" ref="titleInput">
        </div>
        <div>
            <textarea class="writing-pad" ref="writingPad" title="writ content" spellcheck="true" wrap="off" minlength="10" required placeholder="content of your writ (it can be markdown)"></textarea>
        </div>
        <div class="tags-and-toggles">
            <input type="text" name="tags" title="tag input" id="tag-input" placeholder="comma, separated, tags" autocomplete="off" ref="tagInput">
            <div>
                <label for="is-public">public</label>
                <div class="togglebox"> 
                    <input type="checkbox" name="public" id="is-public" ref="isPublicCheckbox" checked>
                    <span></span>
                </div>
            </div>
            <div>
                <label for="is-commentable">commentable</label>
                <div class="togglebox"> 
                    <input type="checkbox" name="commentable" id="is-commentable" ref="isCommentableCheckbox" checked>
                    <span></span>
                </div>
            </div>
        </div>
        <section class="ribbon">
            <button class="submit" ref="pushWritBtn">Push</button>
            <button class="submit" ref="clearEditorBtn">Clear Editor</button>
        </section>
    </section>
    <aside class="writ-selector" ref="writSelector">
        <header>Writs</header>
        <section class="writ-list" ref="writList"></section>
    </aside>
</article>`.collect()

writingPad.value = ''

;(app.wwTS = app.setupToggleSituation(wwLauncher, wwView, 'body', {
    viewOutAnimation: 'fade-out 220ms ease-out',
    delayRemoveMS: 220,
})).toggleView()

const writListEntry = (title, id) => div({
    class: 'wl-entry',
    $: writList,
    attr: {wid: id}
},
    span(title),
    div(
        () => {
            const delBtn = span({
                class: 'delete-writ',
                attr: {
                    title: 'Double click/tap to delete writ'
                }
            }, '🗑')

            // manually jigging double click/tap
            let timeout, clicks = 0
            const clickHandler = d.on.pointerup(delBtn, async e => {
                clearTimeout(timeout)
                if (++clicks == 2) {
                    clicks = 0
                } else {
                    if (clicks == 1) delBtn.classList.add('prep')
                    timeout = setTimeout(() => {
                        delBtn.classList.remove('prep')
                        clicks = 0
                    }, 900)
                    return
                }
                try {
                    clickHandler.off()
                    const res = await app.deleteWritRequest(id)
                    if (res.ok) {
                        df.remove(d.query(`[wid="${id}"]`))
                        if (app.ww.active && app.ww.active.id == id) app.clearEditor()
                        delete app.ww.writs[id]
                    }
                } catch (e) {
                    clickHandler.on()
                    console.error(`Well, that didn't work: ${e}`)
                }
            })

            return delBtn
        }
    )
)

app.ww = {writs: {}}

app.pushWrit = async (title, raw_content, tags, ops = {}) => {
    const raw_writ = {
        title,
        raw_content: raw_content.trim(),
        tags,
        kind: 'post',
        public: true,
        viewable_by: [],
        ...ops,
    }
    const res = await app.jsonPut('/writ', raw_writ)
    const data = await res.json()

    return data.ok ? Promise.resolve(data.data) : Promise.reject(data)
}

app.deleteWritRequest = writID => app.txtDelete('/writ', writID)

app.editableWritQuery({
    author_name: app.user.username,
    with_raw_content: false,
}).then(async writs => {
    if (!d.isArr(writs)) {
        console.error("failed to fetch user's editable writs")
    }
    console.log(writs)

    for (const w of writs) {
        app.ww.writs[w.id] = w
        writListEntry(w.title, w.id)
    }
})

app.rawContentRequest = async wid => {
    const res = await fetch('/writ-raw-content/' + wid)
    return await res.json()
}

d.on.pointerup(writList, e => {
    if (e.target.classList.contains('selected') || e.target.parentElement.classList.contains('selected')) return
    let wid = e.target.getAttribute('wid') || e.target.parentElement.getAttribute('wid')
    if (wid != null) {
        const writ = app.ww.active = app.ww.writs[wid]
        if (app.ww.selectedWLE) app.ww.selectedWLE.classList.remove('selected')
        app.ww.selectedWLE = d.query(`[wid="${wid}"]`)
        app.ww.selectedWLE.classList.add('selected')

        let noContent = writ.raw_content == null
        if (writ.raw_content == null) {
            app.rawContentRequest(writ.id).then(data => {
                if (!data.ok) {
                    writingPad.value = 'Ok, so loading failed for some reason, you should reload or something, it\'s probably a client side error, or less likely, a database issue - ' + (data.status || '')
                }
                writingPad.value = writ.raw_content = data.data
                noContent = false
            })
        }

        titleInput.value = writ.title
        writingPad.value = writ.raw_content || 'Hang tight, the content is loading...'
        setTimeout(() => {
            if (noContent) {
                let tick = 0
                const baseMsg = 'Hang tight, the content is loading'
                let interval = setInterval(() => {
                    if (noContent) {
                        let dots = ''
                        d.each(tick++, () => dots += '.')
                        writingPad.value = baseMsg + dots
                        if (tick == 4) tick = 0
                    } else {
                        clearInterval(interval)
                        tick = 0
                        writingPad.value = writ.raw_content
                    }
                }, 220);
            }
        }, 220)
        tagInput.value = writ.tags.join(', ')
        isPublicCheckbox.checked = writ.commentable
        isCommentableCheckbox.checked = writ.public
        pushWritBtn.textContent = 'Update'
    }
})

app.clearEditor = () => {
    if (app.ww.active) app.ww.active = null
    titleInput.value = writingPad.value = tagInput.value = ''
    isPublicCheckbox.checked = isCommentableCheckbox.checked = true
    if (app.ww.selectedWLE) {
        app.ww.selectedWLE.classList.remove('selected')
        app.ww.selectedWLE = null
        pushWritBtn.textContent = 'Push'
    }
}

app.editorPushWrit = async () => {
    console.log('trying to push writ...')
    let res

    const title = titleInput.value.trim()
    const raw_content = writingPad.value.trim()
    const public = isPublicCheckbox.checked
    const commentable = isCommentableCheckbox.checked
    const tags = tagInput.value.split(',').map(t => t.trim())
    const ops = {
        is_md: true,
        public,
        commentable
    }
    if (app.ww.active) ops.id = app.ww.active.id
    res = await app.pushWrit(title, raw_content, tags, ops)

    if (res != null && res.ok) {
        console.log(res)
        return res
    }
}

d.on.pointerup(clearEditorBtn, app.clearEditor)

d.on.pointerup(pushWritBtn, e => {
    app.editorPushWrit()
})
