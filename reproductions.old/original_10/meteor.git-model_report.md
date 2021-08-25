# Model report for file:///tmp/top-repos-quality-repos-jfbz9yl0/meteor.git HEAD 39ab32f6038b36e680cb3bfb46bec2ee3310a844

### Dump

```json
{'created_at': '2021-08-18 05:07:42',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '36.9 kB',
 'tags': [],
 'uuid': '204e9f03-6ad8-4366-9b9b-1c5f597b08f0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jfbz9yl0/meteor.git 39ab32f6038b36e680cb3bfb46bec2ee3310a844

# javascript
123 rules, avg.len. 14.8
## train
PPCR: 0.871784
### report
macro
{'f1-score': 0.3759704245639043,
 'precision': 0.4475358869605435,
 'recall': 0.3551667772753309,
 'support': 426379}
micro
{'f1-score': 0.968551453049986,
 'precision': 0.968551453049986,
 'recall': 0.968551453049986,
 'support': 426379}
weighted
{'f1-score': 0.9656271663871702,
 'precision': 0.9652818414120563,
 'recall': 0.968551453049986,
 'support': 426379}
### report_full
macro
{'f1-score': 0.30448391521289375,
 'precision': 0.4475358869605435,
 'recall': 0.2666114212571054,
 'support': 489088}
micro
{'f1-score': 0.9022061964003072,
 'precision': 0.968551453049986,
 'recall': 0.8443674757916776,
 'support': 489088}
weighted
{'f1-score': 0.8834104631117818,
 'precision': 0.9599774474073282,
 'recall': 0.8443674757916776,
 'support': 489088}
## test
PPCR: 0.827693
### report
macro
{'f1-score': 0.3267020946481699,
 'precision': 0.3431588479990171,
 'recall': 0.32307935135267024,
 'support': 127507}
micro
{'f1-score': 0.9652411240167207,
 'precision': 0.9652411240167207,
 'recall': 0.9652411240167207,
 'support': 127507}
weighted
{'f1-score': 0.9628169702058582,
 'precision': 0.9632280680731179,
 'recall': 0.9652411240167207,
 'support': 127507}
### report_full
macro
{'f1-score': 0.2647101900770577,
 'precision': 0.3431588479990171,
 'recall': 0.2333536260724691,
 'support': 154051}
micro
{'f1-score': 0.8742426072070408,
 'precision': 0.9652411240167207,
 'recall': 0.7989237330494446,
 'support': 154051}
weighted
{'f1-score': 0.8557182792072251,
 'precision': 0.9567687583096356,
 'recall': 0.7989237330494446,
 'support': 154051}
```

## javascript
### Summary
123 rules, avg.len. 14.8

| | |
|-|-|
|Min support|90|
|Max support|55219|
|Min confidence|0.8008474707603455|
|Max confidence|0.9987212419509888|

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
               'min_samples_leaf_max': 130,
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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {CALL}<br>	∧ -2.length ≥ 6<br>⇒ y = "<br>Confidence: 0.992. Support: 18009.` |
| 2 | `  •••start_line ≥ 90<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ ^2.internal_type = ObjectProperty<br>⇒ y = "<br>Confidence: 0.906. Support: 240.` |
| 3 | `  •••start_line ≥ 90<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ -3.label in {<newline>}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = "<br>Confidence: 0.918. Support: 153.` |
| 4 | `  •••start_line ≥ 90<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ -3.label not in {<newline>}<br>	∧ +5.roles in {VALUE}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = "<br>Confidence: 0.824. Support: 168.` |
| 5 | `  •••start_col ≤ 57<br>	∧ •••start_line ≥ 194<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -4.length ≥ 5<br>	∧ +4.roles in {KEY}<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.internal_type not in {FunctionExpression, ObjectProperty}<br>⇒ y = "<br>Confidence: 0.954. Support: 98.` |
| 6 | `  •••start_col ≤ 57<br>	∧ •••start_line ≤ 248<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -4.length ≥ 5<br>	∧ +4.roles not in {KEY}<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.internal_type not in {FunctionExpression, ObjectProperty}<br>⇒ y = "<br>Confidence: 0.839. Support: 90.` |
| 7 | `  •••start_line ≥ 90<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_col ≤ 13<br>	∧ -4.length ≤ 4<br>	∧ +2.roles in {MAP}<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.internal_type not in {FunctionExpression, ObjectProperty}<br>⇒ y = '<br>Confidence: 0.874. Support: 337.` |
| 8 | `  •••start_line ≥ 90<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 5<br>	∧ ^2.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.815. Support: 739.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 14909.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.849. Support: 521.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.917. Support: 1974.` |
| 12 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 6<br>	∧ -2.diff_offset ≥ 17<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = "<br>Confidence: 0.821. Support: 198.` |
| 13 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = "<br>Confidence: 0.997. Support: 17730.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≤ 5<br>	∧ -5.roles in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 2996.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 5<br>	∧ -3.label not in {<tab>}<br>	∧ -5.roles not in {KEY, LITERAL}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1901.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.944. Support: 1483.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, [}<br>	∧ -1.length ≤ 5<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -4.roles in {STRING}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.823. Support: 167.` |
| 18 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.957. Support: 173.` |
| 19 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {VALUE}<br>⇒ y = "<br>Confidence: 0.898. Support: 162.` |
| 20 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.diff_col ≥ 6<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved = ,<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {CALL} and not in {VALUE}<br>	∧ ^2.roles not in {BODY, VALUE}<br>⇒ y = '<br>Confidence: 0.950. Support: 90.` |
| 21 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ +5.roles in {STRING}<br>	∧ +5.length ≤ 4<br>	∧ ^1.roles not in {VALUE}<br>	∧ ^2.roles not in {BODY, VALUE}<br>⇒ y = "<br>Confidence: 0.864. Support: 114.` |
| 22 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -5.diff_offset ≥ 12<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ +4.reserved = :<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.roles not in {VALUE}<br>	∧ ^2.roles not in {BODY, VALUE}<br>⇒ y = '<br>Confidence: 0.828. Support: 119.` |
| 23 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ +3.roles in {MAP}<br>	∧ ^1.roles not in {VALUE}<br>	∧ ^2.roles not in {BODY, VALUE}<br>⇒ y = '<br>Confidence: 0.918. Support: 176.` |
| 24 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 2<br>	∧ ^2.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.820. Support: 547.` |
| 25 | `  •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type = ExpressionStatement<br>	∧ ^2.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.964. Support: 98.` |
| 26 | `  •••start_col ≤ 14<br>	∧ •••start_line ≥ 90<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 2<br>	∧ ^2.internal_type = ExpressionStatement<br>	∧ ^2.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.811. Support: 98.` |
| 27 | `  •••start_line ≤ 89<br>	∧ -1.diff_col ≥ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<space>}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.874. Support: 147.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 55219.` |
| 29 | `  •••start_line ≥ 238<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.label in {<space>}<br>	∧ -5.length ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +3.reserved not in {[}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.925. Support: 154.` |
| 30 | `  •••start_line ≥ 238<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles in {RIGHT}<br>	∧ -3.label in {<space>}<br>	∧ -5.length ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +3.reserved not in {[}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.842. Support: 263.` |
| 31 | `  •••start_line ≥ 238<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +3.reserved not in {[}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.853. Support: 2148.` |
| 32 | `  •••start_line ≤ 237<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +3.reserved not in {[}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.903. Support: 2947.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 295.` |
| 34 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 1<br>	∧ +3.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.841. Support: 116.` |
| 35 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 1<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎⏎<br>Confidence: 0.868. Support: 1462.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 334.` |
| 37 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ -3.label not in {<space>}<br>	∧ +2.length ≤ 82<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 9041.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1012.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 628.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 566.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles in {BINARY}<br>	∧ ^1.roles in {ARITHMETIC, OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 121.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {ARITHMETIC, OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.828. Support: 679.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {ARITHMETIC, OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.888. Support: 1361.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 215.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 13445.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1568.` |
| 47 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.834. Support: 431.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 4953.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 352.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -5.reserved not in {{}<br>	∧ -5.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 180.` |
| 51 | `  •••start_col ≥ 40<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≤ 5<br>	∧ -2.label in {<space>}<br>	∧ -5.diff_col ≥ 11<br>	∧ -5.label in {<space>}<br>	∧ -5.reserved not in {{}<br>	∧ -5.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.801. Support: 118.` |
| 52 | `  •••start_col ≤ 39<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -5.label in {<space>}<br>	∧ -5.reserved not in {{}<br>	∧ -5.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.802. Support: 139.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.894. Support: 354.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 98.` |
| 55 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 107.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.912. Support: 336.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.917. Support: 442.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 127.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 892.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<tab>}<br>	∧ -3.reserved not in {(, [}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.880. Support: 504.` |
| 62 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<tab>}<br>	∧ -2.length ≤ 1<br>	∧ -3.reserved not in {(, [}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 5074.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<tab>}<br>	∧ -3.reserved not in {(, [}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 44435.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 22050.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 15261.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 9865.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP} and not in {COMMENT}<br>	∧ +5.length ≥ 4<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.865. Support: 197.` |
| 68 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {VALUE}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 113.` |
| 69 | `  •••start_col ≥ 19<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved = }<br>	∧ -5.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {VALUE}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.896. Support: 91.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP} and not in {COMMENT}<br>	∧ +1.length ≥ 10<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.821. Support: 92.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT, MAP}<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 107.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT, MAP}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.955. Support: 143.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.reserved not in {)}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT, MAP}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.887. Support: 411.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT, MAP}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.886. Support: 7044.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 6254.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 6433.` |
| 77 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 3048.` |
| 78 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 1099.` |
| 79 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.907. Support: 123.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.894. Support: 578.` |
| 81 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 130.` |
| 82 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = ArrowFunctionExpression<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.831. Support: 269.` |
| 83 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT, VALUE} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.817. Support: 161.` |
| 84 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.864. Support: 993.` |
| 85 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 86 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], else, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 193.` |
| 87 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⇥<br>Confidence: 0.978. Support: 430.` |
| 88 | `  •••start_col ≥ 32<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ -5.diff_col ≥ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.821. Support: 371.` |
| 89 | `  •••start_col ≥ 32<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 16<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≤ 8<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 642.` |
| 90 | `  •••start_col ≤ 31<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.806. Support: 1090.` |
| 91 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved = {<br>	∧ +3.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.852. Support: 145.` |
| 92 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = {<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 811.` |
| 93 | `  •••start_col ≤ 31<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.879. Support: 617.` |
| 94 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 7<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.905. Support: 460.` |
| 95 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 391.` |
| 96 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.823. Support: 201.` |
| 97 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 196.` |
| 98 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], }}<br>	∧ +1.roles in {NUMBER} and not in {KEY}<br>	∧ +1.length ≤ 7<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 113.` |
| 99 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≤ 7<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.910. Support: 94.` |
| 100 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.length ≤ 2<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≤ 7<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 837.` |
| 101 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ⇥<br>Confidence: 0.962. Support: 302.` |
| 102 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], }}<br>	∧ +1.roles in {STRING} and not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 103 | `  •••start_col ≥ 3<br>	∧ •••start_line ≤ 194<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.815. Support: 203.` |
| 104 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.916. Support: 1028.` |
| 105 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.947. Support: 3370.` |
| 106 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 115.` |
| 107 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1365.` |
| 108 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved = }<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.806. Support: 157.` |
| 109 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1865.` |
| 110 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 540.` |
| 111 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 223.` |
| 112 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 15672.` |
| 113 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles in {IDENTIFIER} and not in {ARGUMENT}<br>	∧ -3.length ≥ 2<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 669.` |
| 114 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT, IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.902. Support: 659.` |
| 115 | `  •••start_col ≥ 24<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>, <tab>}<br>	∧ -3.roles not in {ARGUMENT, IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.847. Support: 291.` |
| 116 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 1<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 11954.` |
| 117 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 2444.` |
| 118 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved = )<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 119 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved = )<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.840. Support: 153.` |
| 120 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ], {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {STATEMENT} and not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.801. Support: 98.` |
| 121 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {CALL}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 3<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.850. Support: 103.` |
| 122 | `  •••start_col ≥ 3<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {CALL}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.840. Support: 2751.` |
| 123 | `  •••start_col ≤ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 336.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 14.8130081300813, "max_conf": 0.9987212419509888, "max_support": 55219, "min_conf": 0.8008474707603455, "min_support": 90, "num_rules": 123}}
```
</details>
