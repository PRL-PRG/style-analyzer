# Model report for file:///tmp/top-repos-quality-repos-j54yfxxz/shakespurrean-twitter-bot.git HEAD 64f7cda4d9f2453041b167f18584558bcd8c08b6

### Dump

```json
{'created_at': '2021-08-21 06:04:01',
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
 'size': '19.8 kB',
 'tags': [],
 'uuid': '693be814-c369-4f68-a295-1aaec977d89d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j54yfxxz/shakespurrean-twitter-bot.git 64f7cda4d9f2453041b167f18584558bcd8c08b6

# javascript
100 rules, avg.len. 9.4
## train
PPCR: 0.933915
### report
macro
{'f1-score': 0.6843753998051102,
 'precision': 0.6989859464812462,
 'recall': 0.6711435240259971,
 'support': 32843}
micro
{'f1-score': 0.9457418628018147,
 'precision': 0.9457418628018147,
 'recall': 0.9457418628018147,
 'support': 32843}
weighted
{'f1-score': 0.9426901704664438,
 'precision': 0.9402567440789051,
 'recall': 0.9457418628018147,
 'support': 32843}
### report_full
macro
{'f1-score': 0.6197170723607167,
 'precision': 0.6989859464812462,
 'recall': 0.57289301585273,
 'support': 35167}
micro
{'f1-score': 0.9134244963975887,
 'precision': 0.9457418628018147,
 'recall': 0.883242812864333,
 'support': 35167}
weighted
{'f1-score': 0.9024025228038262,
 'precision': 0.9335746169977335,
 'recall': 0.883242812864333,
 'support': 35167}
## test
PPCR: 0.939147
### report
macro
{'f1-score': 0.7040595553567819,
 'precision': 0.7252119847338673,
 'recall': 0.6857090636843277,
 'support': 7840}
micro
{'f1-score': 0.9571428571428572,
 'precision': 0.9571428571428572,
 'recall': 0.9571428571428572,
 'support': 7840}
weighted
{'f1-score': 0.9539506855152998,
 'precision': 0.9516837694973321,
 'recall': 0.9571428571428572,
 'support': 7840}
### report_full
macro
{'f1-score': 0.6449029367545518,
 'precision': 0.7252119847338673,
 'recall': 0.594540126302113,
 'support': 8348}
micro
{'f1-score': 0.9271064986409687,
 'precision': 0.9571428571428572,
 'recall': 0.8988979396262577,
 'support': 8348}
weighted
{'f1-score': 0.9169242874537756,
 'precision': 0.9459691988991,
 'recall': 0.8988979396262577,
 'support': 8348}
```

## javascript
### Summary
66 rules, avg.len. 9.4

| | |
|-|-|
|Min support|144|
|Max support|5594|
|Min confidence|0.9201840758323669|
|Max confidence|0.9997760653495789|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 2233.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.990. Support: 620.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.961. Support: 2466.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 5594.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 542.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 4454.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 2016.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 808.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 417.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 289.` |
| 11 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 560.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {BODY, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 144.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BODY, FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 2787.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.label in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.978. Support: 612.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -5.label not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.965. Support: 2408.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 5523.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 519.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 4425.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2130.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.942. Support: 781.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 469.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {BODY} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 561.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {LITERAL} and not in {BODY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 253.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.roles not in {BODY, FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 2467.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.roles not in {BODY, FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 180.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5590.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 514.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 4418.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2097.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 362.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 540.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {LITERAL} and not in {BODY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 237.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 160.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 2597.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 201.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {LITERAL}<br>	∧ -4.diff_col ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 433.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 453.` |
| 39 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 534.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {BLOCK, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 154.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 2467.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 170.` |
| 43 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 567.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BODY, FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 2452.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 408.` |
| 46 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 578.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 167.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {BODY, FILE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 163.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles not in {BODY, FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 2744.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 4415.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 551.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 267.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {BODY, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 162.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {BODY, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 2448.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {BODY, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 190.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {LITERAL}<br>	∧ -4.diff_col ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 453.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 280.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 155.` |
| 59 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 583.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles not in {BODY, FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 2481.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {BODY, FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 193.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 4536.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 559.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {LITERAL} and not in {BODY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 150.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {LITERAL} and not in {BODY, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 196.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.378787878787879, "max_conf": 0.9997760653495789, "max_support": 5594, "min_conf": 0.9201840758323669, "min_support": 144, "num_rules": 66}}
```
</details>
