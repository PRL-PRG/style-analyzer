# Model report for file:///tmp/top-repos-quality-repos-tpnc2wq6/missingdata_backend.git HEAD c5bac73c119058e04be0e0793b8efa63a48723c7

### Dump

```json
{'created_at': '2021-08-24 13:01:02',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '33.0 kB',
 'tags': [],
 'uuid': 'a6cd14b5-ef8a-40a9-bb99-c2a909e000b8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tpnc2wq6/missingdata_backend.git c5bac73c119058e04be0e0793b8efa63a48723c7

# javascript
152 rules, avg.len. 10.3
## train
PPCR: 0.923284
### report
macro
{'f1-score': 0.5258502217316346,
 'precision': 0.5683542467712878,
 'recall': 0.49900969885670343,
 'support': 255482}
micro
{'f1-score': 0.962576619879287,
 'precision': 0.962576619879287,
 'recall': 0.962576619879287,
 'support': 255482}
weighted
{'f1-score': 0.9607915481447741,
 'precision': 0.9600970398063708,
 'recall': 0.962576619879287,
 'support': 255482}
### report_full
macro
{'f1-score': 0.4278335140855739,
 'precision': 0.5683542467712878,
 'recall': 0.37408244315491723,
 'support': 276710}
micro
{'f1-score': 0.9241814984065901,
 'precision': 0.962576619879287,
 'recall': 0.8887318853673521,
 'support': 276710}
weighted
{'f1-score': 0.9154030500685202,
 'precision': 0.9546221212819699,
 'recall': 0.8887318853673521,
 'support': 276710}
## test
PPCR: 0.911938
### report
macro
{'f1-score': 0.5104939283452707,
 'precision': 0.5554434701312926,
 'recall': 0.4838995925343561,
 'support': 70118}
micro
{'f1-score': 0.9528794318149405,
 'precision': 0.9528794318149405,
 'recall': 0.9528794318149405,
 'support': 70118}
weighted
{'f1-score': 0.9507152510468949,
 'precision': 0.9502875168202081,
 'recall': 0.9528794318149405,
 'support': 70118}
### report_full
macro
{'f1-score': 0.40751451229698304,
 'precision': 0.5554434701312926,
 'recall': 0.35201532251484297,
 'support': 76889}
micro
{'f1-score': 0.9089907283326645,
 'precision': 0.9528794318149405,
 'recall': 0.8689669523598954,
 'support': 76889}
weighted
{'f1-score': 0.8983480118173505,
 'precision': 0.9447513857324216,
 'recall': 0.8689669523598954,
 'support': 76889}
```

## javascript
### Summary
97 rules, avg.len. 10.3

| | |
|-|-|
|Min support|94|
|Max support|26552|
|Min confidence|0.9221453070640564|
|Max confidence|0.9997493624687195|

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
| 1 | `  -1.internal_type = CommentLine<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.922. Support: 289.` |
| 2 | `  -1.internal_type = CommentLine<br>	∧ -2.diff_col ≥ 8<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 9816.` |
| 3 | `  -1.internal_type = CommentLine<br>	∧ -4.diff_line = 0<br>	∧ +5.internal_type = StringLiteral<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 451.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ -4.diff_line = 0<br>	∧ -4.reserved not in {;}<br>	∧ -4.length ≤ 5<br>	∧ +1.length ≤ 1<br>	∧ +5.internal_type not in {StringLiteral}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 208.` |
| 5 | `  -1.diff_offset ≥ 6<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.length ≥ 19<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.970. Support: 117.` |
| 6 | `  •••start_col ≤ 28<br>	∧ •••start_line ≥ 217<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.label in {<+space>}<br>	∧ -5.length ≤ 19<br>	∧ ^1.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.995. Support: 305.` |
| 7 | `  •••start_line ≥ 217<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.label not in {<+space>}<br>	∧ -5.length ≤ 19<br>	∧ ^1.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.987. Support: 2110.` |
| 8 | `  •••start_line ≥ 217<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.length ≤ 6<br>	∧ ^1.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.928. Support: 3412.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -2.diff_offset ≥ 10<br>	∧ -3.length ≥ 4<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.999. Support: 417.` |
| 10 | `  •••start_col ≥ 61<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.diff_offset ≥ 10<br>	∧ -3.length ≤ 3<br>	∧ ^1.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.982. Support: 142.` |
| 11 | `  •••start_col ≤ 61<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.diff_offset ≥ 10<br>	∧ -3.length ≤ 3<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = '<br>Confidence: 0.975. Support: 1930.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.reserved = [<br>	∧ ^1.roles not in {CALL}<br>⇒ y = '<br>Confidence: 0.984. Support: 152.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {)}<br>	∧ -1.roles in {RIGHT}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.reserved not in {[}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = "<br>Confidence: 1.000. Support: 1126.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {RIGHT}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.reserved not in {[}<br>	∧ -5.diff_offset ≥ 18<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.948. Support: 454.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 8<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1473.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label in {"} and not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.999. Support: 713.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≥ 19<br>	∧ -5.reserved = )<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 226.` |
| 18 | `  •••start_col ≥ 16<br>	∧ •••start_line ≥ 109<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.diff_offset ≥ 8<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 19<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.975. Support: 1279.` |
| 19 | `  •••start_col ≥ 16<br>	∧ •••start_line ≥ 109<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.diff_offset ≤ 7<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.diff_offset ≥ 21<br>	∧ -4.length ≤ 19<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.940. Support: 210.` |
| 20 | `  •••start_col ≥ 16<br>	∧ •••start_line ≥ 109<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.diff_offset ≤ 7<br>	∧ -2.internal_type = Identifier<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 6<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.939. Support: 585.` |
| 21 | `  •••start_col ≥ 16<br>	∧ •••start_line ≥ 109<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 19<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.968. Support: 3273.` |
| 22 | `  •••start_col ≤ 15<br>	∧ •••start_line ≥ 109<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 19<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 6<br>	∧ +3.length ≥ 7<br>⇒ y = "<br>Confidence: 0.958. Support: 154.` |
| 23 | `  •••start_line ≤ 108<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 19<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = "<br>Confidence: 0.984. Support: 96.` |
| 24 | `  •••start_line ≤ 108<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label not in {", <newline>, <space>}<br>	∧ -2.length ≥ 4<br>	∧ -4.length ≤ 19<br>	∧ -5.diff_offset ≥ 26<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.973. Support: 94.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -3.diff_col ≥ 6<br>	∧ -4.roles not in {BINARY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.length ≤ 7<br>⇒ y = '<br>Confidence: 0.998. Support: 230.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -3.diff_col ≤ 5<br>	∧ -4.roles not in {BINARY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.length ≤ 7<br>⇒ y = '<br>Confidence: 0.969. Support: 2040.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -5.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 226.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -5.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 29 | `  •••start_col ≥ 29<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -5.length ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 307.` |
| 30 | `  •••start_col ≤ 28<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -4.diff_col ≤ 8<br>	∧ -5.diff_col ≤ 17<br>	∧ -5.length ≥ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.995. Support: 102.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {", <space>}<br>	∧ -2.length ≤ 3<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.945. Support: 1544.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_offset ≥ 9<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.internal_type = CallExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 225.` |
| 33 | `  •••start_col ≥ 56<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_offset ≤ 9<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.internal_type = CallExpression<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.996. Support: 138.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {FOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 227.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.reserved not in {)}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {BINARY}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 555.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 354.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 258.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 125.` |
| 40 | `  •••start_col ≤ 22<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles in {FOR, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 138.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 7085.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 129.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 613.` |
| 44 | `  •••start_col ≥ 24<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 507.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION, NUMBER}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 775.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_col ≥ 5<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 9<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION} and not in {NUMBER}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 182.` |
| 47 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_col ≤ 4<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 9<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION} and not in {NUMBER}<br>	∧ ^1.roles in {ADD, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 3628.` |
| 49 | `  •••start_col ≥ 39<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = Identifier<br>	∧ -5.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +3.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.938. Support: 136.` |
| 50 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = Identifier<br>	∧ -5.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +3.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.949. Support: 243.` |
| 51 | `  •••start_col ≤ 26<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = Identifier<br>	∧ -5.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.932. Support: 154.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 8<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -5.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 6<br>	∧ +4.length ≤ 9<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.926. Support: 2067.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 3479.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 2213.` |
| 55 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1669.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.length ≥ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1021.` |
| 57 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.length ≤ 6<br>	∧ -4.length ≤ 11<br>	∧ -5.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {CALL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 113.` |
| 58 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.length ≤ 6<br>	∧ -4.length ≤ 11<br>	∧ -5.diff_col ≥ 9<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {CALL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 317.` |
| 59 | `  •••start_col ≤ 41<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.length ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 104.` |
| 60 | `  •••start_col ≤ 41<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.diff_col ≥ 7<br>	∧ -4.length ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 216.` |
| 61 | `  •••start_col ≤ 41<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.diff_col ≤ 6<br>	∧ -4.length ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 589.` |
| 62 | `  •••start_col ≤ 41<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_col ≤ 3<br>	∧ -4.length ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 114.` |
| 63 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.length ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 389.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +5.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 827.` |
| 65 | `  •••start_col ≤ 39<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.diff_offset ≥ 8<br>	∧ -3.length ≤ 7<br>	∧ -5.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.length ≥ 7<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.996. Support: 116.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.length ≥ 11<br>	∧ -5.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 416.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≥ 11<br>	∧ -4.length ≤ 10<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ -5.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.977. Support: 111.` |
| 68 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.reserved = (<br>	∧ -4.length ≤ 10<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.998. Support: 225.` |
| 69 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.reserved = )<br>	∧ -4.length ≤ 10<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.996. Support: 114.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label in {"}<br>	∧ -4.reserved not in {)}<br>	∧ -4.length ≤ 10<br>	∧ -5.label in {<space>}<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +2.length ≥ 5<br>	∧ +5.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 113.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.reserved not in {)}<br>	∧ -4.length ≤ 10<br>	∧ -5.label not in {<space>}<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +2.length ≥ 5<br>	∧ +5.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 880.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.reserved not in {)}<br>	∧ -4.length ≤ 10<br>	∧ -5.reserved not in {(}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +2.length ≤ 4<br>	∧ +5.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 1807.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 798.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 184.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles in {FUNCTION}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≥ 2<br>	∧ -5.label not in {<space>}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1370.` |
| 76 | `  •••start_col ≥ 96<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_col ≥ 6<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 77 | `  •••start_col ≤ 95<br>	∧ •••start_line ≥ 50<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 9345.` |
| 78 | `  •••start_col ≤ 95<br>	∧ •••start_line ≤ 49<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 79 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = typeof<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 628.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {NAME}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 460.` |
| 81 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {NAME}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.995. Support: 99.` |
| 82 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = if<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 373.` |
| 83 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = if<br>	∧ +4.length ≤ 13<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 222.` |
| 84 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = +<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 172.` |
| 85 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, typeof, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 166.` |
| 86 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -3.diff_col ≥ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.length ≥ 30<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.943. Support: 96.` |
| 87 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +1.roles in {MAP}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 316.` |
| 88 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -5.diff_offset ≥ 15<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1971.` |
| 89 | `  •••start_col ≤ 35<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -5.diff_col ≥ 13<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.length ≤ 29<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 160.` |
| 90 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -5.diff_col ≤ 12<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 260.` |
| 91 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +1.length ≤ 4<br>	∧ +2.length ≤ 29<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 4562.` |
| 92 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {, }}<br>	∧ -3.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 233.` |
| 93 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {, }}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles in {FOR} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 453.` |
| 94 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.length ≤ 16<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 874.` |
| 95 | `  •••start_line ≥ 14<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, var, {, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, {, }}<br>	∧ +2.length ≤ 29<br>	∧ +4.reserved = function<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1995.` |
| 96 | `  •••start_line ≥ 14<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, var, {, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, {, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 26552.` |
| 97 | `  •••start_line ≤ 13<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, typeof, var, {, }}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {+, {, }}<br>	∧ +2.length ≤ 29<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 788.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.31958762886598, "max_conf": 0.9997493624687195, "max_support": 26552, "min_conf": 0.9221453070640564, "min_support": 94, "num_rules": 97}}
```
</details>
