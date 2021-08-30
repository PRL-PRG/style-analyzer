# Model report for file:///tmp/top-repos-quality-repos-v5rybca5/akshay23sept.github.io.git HEAD a5dedc1a3efe295a5c8b01433a6d41f32c82221b

### Dump

```json
{'created_at': '2021-08-29 05:17:11',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '20.0 kB',
 'tags': [],
 'uuid': '9412f934-d9d2-4cab-a017-124ee13d5c9a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v5rybca5/akshay23sept.github.io.git a5dedc1a3efe295a5c8b01433a6d41f32c82221b

# javascript
41 rules, avg.len. 8.8
## train
PPCR: 0.951513
### report
macro
{'f1-score': 0.6894153312258342,
 'precision': 0.6939122603343183,
 'recall': 0.685307008357093,
 'support': 84736}
micro
{'f1-score': 0.9809526057401813,
 'precision': 0.9809526057401813,
 'recall': 0.9809526057401813,
 'support': 84736}
weighted
{'f1-score': 0.9790446484654555,
 'precision': 0.9772119879507474,
 'recall': 0.9809526057401813,
 'support': 84736}
### report_full
macro
{'f1-score': 0.6276493734717739,
 'precision': 0.6939122603343183,
 'recall': 0.5958144111017527,
 'support': 89054}
micro
{'f1-score': 0.9565797801944876,
 'precision': 0.9809526057401813,
 'recall': 0.9333887304332203,
 'support': 89054}
weighted
{'f1-score': 0.9510828947578412,
 'precision': 0.9728352951007649,
 'recall': 0.9333887304332203,
 'support': 89054}
## test
PPCR: 0.963251
### report
macro
{'f1-score': 0.3315240915516523,
 'precision': 0.3575346629880813,
 'recall': 0.45124691224519575,
 'support': 2726}
micro
{'f1-score': 0.8892149669845927,
 'precision': 0.8892149669845928,
 'recall': 0.8892149669845928,
 'support': 2726}
weighted
{'f1-score': 0.893532863828155,
 'precision': 0.9220456232881971,
 'recall': 0.8892149669845928,
 'support': 2726}
### report_full
macro
{'f1-score': 0.32603185892423237,
 'precision': 0.3575346629880813,
 'recall': 0.4194603748839949,
 'support': 2830}
micro
{'f1-score': 0.8725701943844493,
 'precision': 0.8892149669845928,
 'recall': 0.8565371024734982,
 'support': 2830}
weighted
{'f1-score': 0.875874921109773,
 'precision': 0.922352288445232,
 'recall': 0.8565371024734982,
 'support': 2830}
```

## javascript
### Summary
31 rules, avg.len. 8.9

| | |
|-|-|
|Min support|92|
|Max support|14310|
|Min confidence|0.9335368871688843|
|Max confidence|0.9997645020484924|

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
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 297.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 315.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 193.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 7949.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles in {BINARY}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 142.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1857.` |
| 7 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 352.` |
| 8 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 14310.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4973.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.963. Support: 2119.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.934. Support: 2129.` |
| 12 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 3255.` |
| 13 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 905.` |
| 14 | `  -1.diff_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 157.` |
| 15 | `  -1.diff_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 100.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 224.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 136.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 9583.` |
| 19 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2123.` |
| 20 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 1222.` |
| 21 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.length ≥ 27<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 1127.` |
| 22 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1101.` |
| 23 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 92.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 4086.` |
| 25 | `  -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BLOCK} and not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 493.` |
| 26 | `  -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 102.` |
| 27 | `  -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 987.` |
| 28 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 115.` |
| 29 | `  -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 164.` |
| 30 | `  -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.diff_col ≥ 8<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 302.` |
| 31 | `  -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3272.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.870967741935484, "max_conf": 0.9997645020484924, "max_support": 14310, "min_conf": 0.9335368871688843, "min_support": 92, "num_rules": 31}}
```
</details>
