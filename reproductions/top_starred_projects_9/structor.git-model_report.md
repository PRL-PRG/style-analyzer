# Model report for file:///tmp/top-repos-quality-repos-213yo6dn/structor.git HEAD f0bf3aab11f03899b477b66c033158a73470ea3d

### Dump

```json
{'created_at': '2021-08-29 13:21:38',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.4 kB',
 'tags': [],
 'uuid': '205add8c-3fdb-4b45-b61a-52f5a86b68a1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-213yo6dn/structor.git f0bf3aab11f03899b477b66c033158a73470ea3d

# javascript
60 rules, avg.len. 9.5
## train
PPCR: 0.897701
### report
macro
{'f1-score': 0.5542582992616231,
 'precision': 0.6233157292987684,
 'recall': 0.517549635241063,
 'support': 83690}
micro
{'f1-score': 0.9562910742024137,
 'precision': 0.9562910742024137,
 'recall': 0.9562910742024137,
 'support': 83690}
weighted
{'f1-score': 0.9516055807999606,
 'precision': 0.9497507935680883,
 'recall': 0.9562910742024137,
 'support': 83690}
### report_full
macro
{'f1-score': 0.4610524467062742,
 'precision': 0.6233157292987684,
 'recall': 0.4159313873210075,
 'support': 93227}
micro
{'f1-score': 0.9047406410915854,
 'precision': 0.9562910742024137,
 'recall': 0.8584637497720617,
 'support': 93227}
weighted
{'f1-score': 0.8806126453523995,
 'precision': 0.9268937923559234,
 'recall': 0.8584637497720617,
 'support': 93227}
## test
PPCR: 0.892826
### report
macro
{'f1-score': 0.5078444781871099,
 'precision': 0.6136693688923368,
 'recall': 0.47647176317338974,
 'support': 20560}
micro
{'f1-score': 0.941147859922179,
 'precision': 0.941147859922179,
 'recall': 0.941147859922179,
 'support': 20560}
weighted
{'f1-score': 0.933458032846198,
 'precision': 0.9337752680330224,
 'recall': 0.941147859922179,
 'support': 20560}
### report_full
macro
{'f1-score': 0.4392897519840126,
 'precision': 0.6136693688923368,
 'recall': 0.40067203743265106,
 'support': 23028}
micro
{'f1-score': 0.8878590437735157,
 'precision': 0.941147859922179,
 'recall': 0.8402813965607087,
 'support': 23028}
weighted
{'f1-score': 0.8612901003927701,
 'precision': 0.9191701885989935,
 'recall': 0.8402813965607087,
 'support': 23028}
```

## javascript
### Summary
39 rules, avg.len. 9.2

| | |
|-|-|
|Min support|96|
|Max support|12938|
|Min confidence|0.921841561794281|
|Max confidence|0.9997222423553467|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 8360.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.922. Support: 467.` |
| 3 | `  •••start_line ≥ 2<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.956. Support: 2342.` |
| 4 | `  •••start_line ≤ 1<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 278.` |
| 5 | `  •••start_col ≥ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.958. Support: 252.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2834.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.947. Support: 1788.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 200.` |
| 9 | `  •••start_col ≤ 34<br>	∧ •••start_line ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_col ≥ 12<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 96.` |
| 10 | `  •••start_col ≤ 34<br>	∧ •••start_line ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_col ≤ 11<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 634.` |
| 11 | `  •••start_line ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 145.` |
| 12 | `  •••start_line ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎⏎<br>Confidence: 0.938. Support: 201.` |
| 13 | `  •••start_line ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 8541.` |
| 14 | `  •••start_line ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = "<br>Confidence: 0.996. Support: 136.` |
| 15 | `  •••start_line ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 360.` |
| 16 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 750.` |
| 17 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.roles in {IMPORT}<br>	∧ +1.reserved not in {import, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.977. Support: 152.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1800.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 988.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 546.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.922. Support: 122.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 369.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 24 | `  •••start_line ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {,, =}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 252.` |
| 25 | `  •••start_line ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {,, =, >}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {FunctionDeclaration}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 2773.` |
| 26 | `  •••start_line ≤ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {,, =}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 224.` |
| 27 | `  •••start_line ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 1931.` |
| 28 | `  •••start_line ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 142.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {CALLEE}<br>	∧ -1.length ≥ 2<br>	∧ -3.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 127.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≤ 1<br>	∧ -3.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = function<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.984. Support: 96.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 817.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 197.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = export<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 34 | `  •••start_line ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 164.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {;}<br>	∧ -5.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 460.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, export}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 9<br>	∧ -3.reserved not in {), ;}<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 12938.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, export}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≤ 8<br>	∧ -3.reserved not in {), ;}<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 413.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, export}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≤ 8<br>	∧ -3.reserved not in {), ;}<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 4807.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.153846153846153, "max_conf": 0.9997222423553467, "max_support": 12938, "min_conf": 0.921841561794281, "min_support": 96, "num_rules": 39}}
```
</details>
